# Open projects

This is a repository of projects that you can chip in on.
These projects are all designed so you can get up and running quickly.
They’re easy to start, easy to stop, easy to resume.
They are great whether you're looking to make your first changes, looking to
spend some idle time doing something useful, want to ramp up your community
contributions, or you're just feeling generous.

Last but not least, each project makes a great starting point for a team fixit.

## Choosing a project

Pick a project from one of the categories below.
You could focus on an area that you're familiar with, or choose something that
you'd like to learn more about.

{% set tocmeta | yamlloads %}
{% include "docs/contribute/open_projects/_toc.yaml" %}
{% endset %}

{% for item in tocmeta.toc %}
  {% if item.path and item.title != "Introduction" and item.title != "Project template" %}
    <li><a href="{{ item.path }}">{{ item.title }}</a></li>
  {% elif item.section and item.title != "Graduated projects" %}
      <h3>{{ item.title }}</h3>
        <ul>
    {% for sectionItem in item.section %}
        {% if sectionItem.path %}
          <li><a href="{{ sectionItem.path }}">{{ sectionItem.title }}</a></li>
        {% endif %}
    {% endfor %}
        </ul>
  {% endif %}
{% endfor %}

## Adding new projects

Anyone can add a project by sending a change to introduce new pages and sections
under this directory. Consult the `OWNERS` file to find someone to review and
approve your change. Reviewers will typically suggest helpful tips for making
your project more effective, but don't act as gatekeepers. Gates open, come on
in.

You'll have an easier time if you start by cloning the [template](template.md).

## Becoming a reviewer

If you're like to help review new projects then please add yourself to the
`OWNERS` file and send the change to one of the existing owners.
