
import { LitElement, html, css } from 'lit';
import type { Project } from '../models/project.js';

class ProjectCard extends LitElement {
  static properties = {
    project: { attribute: false },
  };

  declare project: Project;

  
static styles = css`
  .project {
    padding: 20px;
    margin-bottom: 15px;
    border: 1px solid #e5e7eb;
    border-radius: 10px;
    background: #ffffff;
    color: #1f2937;
    transition: box-shadow 0.2s ease;
  }

  .project:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  h2 {
    margin-top: 0;
    color: #111827;
    font-size: 22px;
  }

  p {
    color: #374151;
    line-height: 1.5;
  }

  .technologies {
    color: #6b7280;
    font-size: 14px;
  }

  a {
    color: #2563eb;
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }
`;


  render() {
    if (!this.project) {
      return html``;
    }

    return html`
      <article class="project">
        <h2>${this.project.title}</h2>

        <p>
          ${this.project.description || 'No description available.'}
        </p>

        <p class="technologies">
          Technologies:
          ${this.project.technologies?.length
        ? this.project.technologies.join(', ')
        : 'Not specified'}
        </p>

        ${this.project.github_url
        ? html`
              <a
                href=${this.project.github_url}
                target="_blank"
                rel="noopener noreferrer"
              >
                View on GitHub →
              </a>
            `
        : 'No Github Repo'}
      </article>
    `;
  }
}

if (!customElements.get('project-card')) {
  customElements.define('project-card', ProjectCard);
}
