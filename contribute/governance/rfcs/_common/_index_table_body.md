  <tr>
    {% setvar full_title %}{{ rfc.name }}: {{ rfc.title }}{% endsetvar %}
    <td><p>{{ rfc.name }}<a name="{{ full_title|replace(" ", "-")|replace("(", "")|replace(")", "")|lower() }}"></a></p><h3 style="display:none">{{ full_title }}</h3></td>
    <td>
        <p>
          <a href="{{ rfc.file }}">{{ rfc.title }}</a>
        </p>
      </td>
      <td>
        <ul class="comma-list">
        {% for area in rfc.area %}
          <li>{{ area }}</li>
        {% endfor %}
        </ul>
      <td>
        <ul class="comma-list">
        {% for change in rfc.gerrit_change_id %}
          <li><a href="{{ gerrit_change_url }}{{ change }}">{{ change }}</a></li>
        {% endfor %}
        </ul>
    </td>
  </tr>
