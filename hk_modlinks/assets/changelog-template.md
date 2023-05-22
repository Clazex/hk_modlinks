# Changelog

{{#if new}}

## New mods

{{#each new ~}}

### {{@key}}

- Description: {{this.description}}
{{#if this.dependencies}}
- Dependencies
{{#each this.dependencies}}
  + {{this}}
{{/each}}
{{/if}}
{{#if this.integrations}}
- Integrations
{{#each this.integrations}}
  + {{this}}
{{/each}}
{{/if}}
{{#if this.tags}}
- Tags
{{#each this.tags}}
  + {{this}}
{{/each}}
{{/if}}

{{/each}}
{{/if~}}

{{#if removed}}

## Removed mods

{{#each removed}}
- {{this}}
{{/each}}

{{/if~}}

{{#if updated}}

## Updated mods

{{#each updated ~}}

### {{@key}}

{{#if this.version}}
- Version: {{this.version.old}} -> {{this.version.new}}
{{/if}}
{{#if this.description}}
- Description
  + Old: {{this.description.old}}
  + New: {{this.description.new}}
{{/if}}
{{#if this.dependencies}}
- Dependencies
{{#if this.dependencies.removed}}
  + Removed
{{#each this.dependencies.removed}}
    - {{this}}
{{/each}}
{{/if}}
{{#if this.dependencies.added}}
  + Added
{{#each this.dependencies.added}}
    - {{this}}
{{/each}}
{{/if}}
{{/if}}
{{#if this.integrations}}
- Integrations
{{#if this.integrations.removed}}
  + Removed
{{#each this.integrations.removed}}
    - {{this}}
{{/each}}
{{/if}}
{{#if this.integrations.added}}
  + Added
{{#each this.integrations.added}}
    - {{this}}
{{/each}}
{{/if}}
{{/if}}
{{#if this.tags}}
- Tags
{{#if this.tags.removed}}
  + Removed
{{#each this.tags.removed}}
    - {{this}}
{{/each}}
{{/if}}
{{#if this.tags.added}}
  + Added
{{#each this.tags.added}}
    - {{this}}
{{/each}}
{{/if}}
{{/if}}

{{/each}}
{{/if~}}
