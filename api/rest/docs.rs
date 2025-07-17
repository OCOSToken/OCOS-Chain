//! OCOS-Chain: REST API Documentation (OpenAPI/Swagger)
//!
//! Provides auto-generated OpenAPI/Swagger schema for API explorer and client codegen.

use axum::response::Html;

// Example: Serve Swagger UI at /docs endpoint (in real code, serve OpenAPI JSON/YAML)
pub async fn openapi_spec() -> Html<&'static str> {
    // In production, serve generated OpenAPI spec file or serve Swagger UI with OpenAPI link.
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
      <title>OCOS-Chain API Docs</title>
      <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist/swagger-ui.css" />
    </head>
    <body>
      <div id="swagger-ui"></div>
      <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
      <script>
        const ui = SwaggerUIBundle({
          url: '/openapi.json', // In production, serve OpenAPI spec at this path
          dom_id: '#swagger-ui',
        });
      </script>
    </body>
    </html>
    "#)
}
