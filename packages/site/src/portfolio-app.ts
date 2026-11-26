import { LitElement, html, css } from 'lit';
import type { Project } from './models/project.js';

// Components
import './components/project-card.js';

class PortfolioApp extends LitElement {
  // Tells Lit which properties should trigger rendering.
  static properties = {
    projects: { state: true }, // state: true - Makes a property reactive without reflecting it to an HTML attribute.
    loading: { state: true },
    error: { state: true },
  };

  declare projects: Project[]; // Tells TypeScript that these properties are defined elsewhere by Lit.
  declare loading: boolean;
  declare error: string;

  // Initializes the properties when the component is created.
  constructor() {
    super();

    this.projects = [];
    this.loading = true;
    this.error = '';
  }

  static styles = css`
  :host {
    display: block;
    min-height: 100vh;
    background: #0f172a;
    color: #f8fafc;
    font-family: Arial, sans-serif;
  }

  * {
    box-sizing: border-box;
  }

  .container {
    width: min(1100px, 90%);
    margin: 0 auto;
  }

  /* Navigation */
  nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 25px 0;
    border-bottom: 1px solid #334155;
  }

  .logo {
    font-size: 22px;
    font-weight: bold;
  }

  nav a {
    color: #cbd5e1;
    text-decoration: none;
    margin-left: 25px;
  }

  nav a:hover {
    color: #38bdf8;
  }

  /* Hero */
  .hero {
    padding: 110px 0;
    max-width: 750px;
  }

  .hero .intro {
    color: #38bdf8;
    font-size: 18px;
  }

  .hero h1 {
    font-size: clamp(38px, 6vw, 64px);
    margin: 15px 0;
  }

  .hero p {
    color: #94a3b8;
    font-size: 18px;
    line-height: 1.7;
  }

  .button {
    display: inline-block;
    margin-top: 20px;
    padding: 13px 22px;
    background: #0284c7;
    color: white;
    text-decoration: none;
    border-radius: 8px;
  }

  .button:hover {
    background: #0369a1;
  }

  /* Projects */
  .projects {
    padding: 60px 0 100px;
  }

  .section-title {
    font-size: 32px;
    margin-bottom: 30px;
  }

  .project-list {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 20px;
  }

  .status {
    color: #94a3b8;
  }

  @media (max-width: 600px) {
    nav {
      flex-direction: column;
      gap: 20px;
    }

    nav a {
      margin: 0 8px;
    }

    .hero {
      padding: 75px 0;
    }
  }
`;

  connectedCallback() {
    super.connectedCallback();
    this.loadProjects();
  }

  async loadProjects() {
    try {
      const response = await fetch('/api/projects');

      if (!response.ok) {
        throw new Error('Failed to load projects');
      }

      this.projects = await response.json();
    } catch (err) {
      this.error = 'Could not connect to the portfolio API.';
      console.error(err);
    } finally {
      this.loading = false;
    }
  }

  render() {
    return html`
    <div class="container">

      <!-- Navigation -->
      <nav>
        <div class="logo">Jazz.dev</div>

        <div>
          <a href="#home">Home</a>
          <a href="#projects">Projects</a>
        </div>
      </nav>

      <!-- Hero section -->
      <section class="hero" id="home">
        <span class="intro">Hello, I'm Jazz</span>

        <h1>
          IT Developer Apprentice
          <br />
          Building for the Web.
        </h1>

        <p>
          I'm an IT development apprentice at Drammen kommune.
          I enjoy working with backend development, web technologies
          and building practical software solutions.
        </p>

        <a href="#projects" class="button">
          Explore my projects
        </a>
      </section>

      <!-- Projects section -->
      <section class="projects" id="projects">
        <h2 class="section-title">My Projects</h2>

        ${this.loading
        ? html`<p class="status">Loading projects...</p>`
        : ''}

        ${this.error
        ? html`<p class="status">${this.error}</p>`
        : ''}

        ${!this.loading && !this.error
        ? html`
              <div class="project-list">
                ${this.projects.map(
          (project) => html`
                    <project-card .project=${project}></project-card>
                  `
        )}
              </div>
            `
        : ''}
      </section>

    </div>
  `;
  }
}

customElements.define('portfolio-app', PortfolioApp);