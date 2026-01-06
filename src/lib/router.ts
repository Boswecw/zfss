/**
 * ZFSS Simple Hash Router
 *
 * Lightweight hash-based routing for the ZFSS application.
 */

export type RouteHandler = (params: Record<string, string>) => void;

interface Route {
  pattern: RegExp;
  paramNames: string[];
  handler: RouteHandler;
}

class Router {
  private routes: Route[] = [];
  private notFoundHandler: RouteHandler = () => {};

  /**
   * Register a route
   * @param path - Path pattern (e.g., "/issues/:id")
   * @param handler - Handler function
   */
  on(path: string, handler: RouteHandler): this {
    const paramNames: string[] = [];
    const patternStr = path.replace(/:([^/]+)/g, (_, name) => {
      paramNames.push(name);
      return "([^/]+)";
    });
    const pattern = new RegExp(`^${patternStr}$`);
    this.routes.push({ pattern, paramNames, handler });
    return this;
  }

  /**
   * Set handler for unknown routes
   */
  notFound(handler: RouteHandler): this {
    this.notFoundHandler = handler;
    return this;
  }

  /**
   * Navigate to a path
   */
  navigate(path: string): void {
    window.location.hash = path;
  }

  /**
   * Get current path from hash
   */
  getCurrentPath(): string {
    return window.location.hash.slice(1) || "/";
  }

  /**
   * Resolve current route
   */
  resolve(): void {
    const path = this.getCurrentPath();

    for (const route of this.routes) {
      const match = path.match(route.pattern);
      if (match) {
        const params: Record<string, string> = {};
        route.paramNames.forEach((name, index) => {
          params[name] = match[index + 1];
        });
        route.handler(params);
        return;
      }
    }

    this.notFoundHandler({});
  }

  /**
   * Start listening to hash changes
   */
  start(): void {
    window.addEventListener("hashchange", () => this.resolve());
    this.resolve();
  }
}

export const router = new Router();

/**
 * Create a navigation link element
 */
export function navLink(
  path: string,
  text: string,
  className: string = ""
): HTMLAnchorElement {
  const link = document.createElement("a");
  link.href = `#${path}`;
  link.textContent = text;
  link.className = className;

  // Add active class if current route
  if (router.getCurrentPath() === path) {
    link.classList.add("active");
  }

  return link;
}

/**
 * Helper to create elements
 */
export function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  attrs?: Record<string, string>,
  children?: (Node | string)[]
): HTMLElementTagNameMap[K] {
  const element = document.createElement(tag);
  if (attrs) {
    Object.entries(attrs).forEach(([key, value]) => {
      if (key === "className") {
        element.className = value;
      } else if (key.startsWith("data-")) {
        element.setAttribute(key, value);
      } else {
        (element as any)[key] = value;
      }
    });
  }
  if (children) {
    children.forEach((child) => {
      if (typeof child === "string") {
        element.appendChild(document.createTextNode(child));
      } else {
        element.appendChild(child);
      }
    });
  }
  return element;
}

/**
 * Clear and set content of an element
 */
export function setContent(container: HTMLElement, content: Node | Node[]): void {
  container.innerHTML = "";
  if (Array.isArray(content)) {
    content.forEach((node) => container.appendChild(node));
  } else {
    container.appendChild(content);
  }
}
