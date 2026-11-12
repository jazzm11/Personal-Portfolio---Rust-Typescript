import { LitElement, html, css } from 'lit';
import type { Project } from './models/project.js';

class PortfolioApp extends LitElement {
  projects: Project[] = [];
  loading = true;
  error = '';
    
  static styles = css`
    :host {
      display: block;
      font-family: Arial, sans-serif;
      padding: 30px;
    }

    .project {
      padding: 20px;
      margin-bottom: 15px;
      border: 1px solid #ddd;
      border-radius: 10px;
    }

    .technologies {
      color: #666;
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
      this.requestUpdate();
    }
  }

  render() {
    if (this.loading) {
      return html`<p>Loading projects...</p>`;
    }

    if (this.error) {
      return html`<p>${this.error}</p>`;
    }

    return html`
      <h1>My Portfolio</h1>

      ${this.projects.map(
        (project) => html`
          <div class="project">
            <h2>${project.title}</h2>
            <p>${project.description}</p>

            <p class="technologies">
              Technologies: ${project.technologies.join(', ')}
            </p>

            ${project.github_url
              ? html`<a href=${project.github_url}>GitHub</a>`
              : ''}
          </div>
        `
      )}
    `;
  }
}

customElements.define('portfolio-app', PortfolioApp);