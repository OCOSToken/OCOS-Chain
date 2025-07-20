// Professional WebSocket service for OCOS-Chain dashboard

type WSMessageHandler = (msg: any) => void;
type WSStatusHandler = (status: "open" | "closed" | "error" | "reconnecting") => void;

export interface WSOptions {
  endpoint?: string;
  protocols?: string | string[];
  onMessage?: WSMessageHandler;
  onStatus?: WSStatusHandler;
  autoReconnect?: boolean;
  reconnectIntervalMs?: number;
  token?: string; // Optional: For auth
}

/**
 * WebSocketClient
 * Basic, extensible WebSocket client for live OCOS-Chain data.
 */
export class WebSocketClient {
  private ws: WebSocket | null = null;
  private endpoint: string;
  private protocols?: string | string[];
  private onMessage?: WSMessageHandler;
  private onStatus?: WSStatusHandler;
  private autoReconnect: boolean;
  private reconnectIntervalMs: number;
  private token?: string;
  private reconnectTimeout: any;

  constructor(options: WSOptions) {
    this.endpoint = options.endpoint || "wss://ws.ocos.io";
    this.protocols = options.protocols;
    this.onMessage = options.onMessage;
    this.onStatus = options.onStatus;
    this.autoReconnect = options.autoReconnect ?? true;
    this.reconnectIntervalMs = options.reconnectIntervalMs ?? 3500;
    this.token = options.token;
  }

  connect() {
    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) return;
    const url = this.token ? `${this.endpoint}?token=${this.token}` : this.endpoint;
    this.ws = this.protocols
      ? new WebSocket(url, this.protocols)
      : new WebSocket(url);

    this.ws.onopen = () => {
      this.onStatus?.("open");
      // Optional: Send initial subscribe message(s) here
    };
    this.ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data);
        this.onMessage?.(msg);
      } catch {
        this.onMessage?.(event.data);
      }
    };
    this.ws.onclose = () => {
      this.onStatus?.("closed");
      if (this.autoReconnect) {
        this.onStatus?.("reconnecting");
        this.reconnectTimeout = setTimeout(() => this.connect(), this.reconnectIntervalMs);
      }
    };
    this.ws.onerror = (e) => {
      this.onStatus?.("error");
      this.ws?.close();
    };
  }

  disconnect() {
    if (this.reconnectTimeout) clearTimeout(this.reconnectTimeout);
    this.ws?.close();
    this.ws = null;
  }

  send(msg: any) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(typeof msg === "string" ? msg : JSON.stringify(msg));
    }
  }

  subscribe(topic: string, params?: any) {
    this.send({ type: "subscribe", topic, ...params });
  }

  unsubscribe(topic: string) {
    this.send({ type: "unsubscribe", topic });
  }
}
