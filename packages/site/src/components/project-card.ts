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
      border: 1px solid #ddd;
      border-radius: 10px;
    }

    .technologies {
      color: #666;
    }
  `;

  render() {
    return html`
      <article class="project">
        <h2>${this.project.title}</h2>
        <p>${this.project.description}</p>

        <p class="technologies">
          Technologies: ${this.project.technologies.join(', ')}
        </p>

        ${this.project.github_url
          ? html`<a href=${this.project.github_url}>GitHub</a>`
          : ''}
      </article>
    `;
  }
}

if (!customElements.get('project-card')) {
  customElements.define('project-card', ProjectCard);
}