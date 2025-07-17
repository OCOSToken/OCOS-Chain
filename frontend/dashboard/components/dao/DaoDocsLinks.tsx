import React from "react";
import {
  BookOpen,
  FileText,
  Github,
  ShieldCheck,
  Globe,
  Link as LinkIcon,
} from "lucide-react";

export interface DaoDocLink {
  label: string;
  url: string;
  icon?: React.ReactNode;
  description?: string;
}

export interface DaoDocsLinksProps {
  links: DaoDocLink[];
  className?: string;
}

const iconMap: Record<string, React.ReactNode> = {
  Whitepaper: <FileText className="w-5 h-5 text-primary" />,
  Docs: <BookOpen className="w-5 h-5 text-accent" />,
  Constitution: <ShieldCheck className="w-5 h-5 text-green-700" />,
  GitHub: <Github className="w-5 h-5 text-gray-700 dark:text-gray-200" />,
  Website: <Globe className="w-5 h-5 text-accent" />,
  Default: <LinkIcon className="w-5 h-5 text-muted-foreground" />,
};

export const DaoDocsLinks: React.FC<DaoDocsLinksProps> = ({ links, className }) => (
  <section className={`bg-card border rounded-2xl shadow px-6 py-5 w-full max-w-lg mx-auto ${className || ""}`}>
    <h3 className="font-bold text-lg mb-3 flex items-center gap-2">
      <BookOpen className="w-5 h-5 text-accent" /> DAO Documents & Links
    </h3>
    <ul className="flex flex-col gap-3">
      {links.length === 0 ? (
        <li className="text-center text-muted-foreground">No documents or resources available.</li>
      ) : (
        links.map((link, idx) => (
          <li key={link.url + idx}>
            <a
              href={link.url}
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 px-2 py-2 rounded-lg hover:bg-muted/60 transition"
              aria-label={link.label}
            >
              {link.icon || iconMap[link.label] || iconMap.Default}
              <span className="font-semibold">{link.label}</span>
              {link.description && (
                <span className="text-xs text-muted-foreground ml-2">{link.description}</span>
              )}
            </a>
          </li>
        ))
      )}
    </ul>
  </section>
);
