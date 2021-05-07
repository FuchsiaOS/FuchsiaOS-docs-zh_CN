<table class="responsive">
  <tbody>
    <tr>
      <th colspan=2>{{ rfc.name }}: {{ rfc.title }}</th>
    </tr>
    {% if rfc.status != "" %}
    <tr>
      <td class="col-key">Status</td><td>{{ rfc.status }}</td>
    </tr>
    {% endif %}
    {% if rfc.area != [""] %}
    <tr>
      <td class="col-key">Areas</td>
      <td>
               <ul class="comma-list">
                 {% for area in rfc.area %}
                 <li>{{ area }}</li>
                 {% endfor %}
               </ul>
       </td>
     </tr>
    {% endif %}
    {% if rfc.short_description != "" %}
    <tr>
      <td class="col-key">Description</td><td><p>{{ rfc.short_description }}</p></td>
    </tr>
    {% endif %}
    <tr>
    {% if rfc.issue != [""] %}
    <tr>
      <td class="col-key">Issues</td>
      <td>
        <ul class="comma-list">
          {% for issue in rfc.issue %}
          <li><a href="{{ issue_url }}{{ issue }}">{{ issue }}</a></li>
          {% endfor %}
        </ul>
      </td>
    </tr>
    {% endif %}
    {% if rfc.gerrit_change_id != [""] %}
    <tr>
      <td class="col-key">Gerrit change</td>
      <td>
        <ul class="comma-list">
          {% for cl in rfc.gerrit_change_id %}
          <li><a href="{{ gerrit_change_url }}{{ cl }}">{{ cl }}</a></li>
          {% endfor %}
        </ul>
      </td>
    </tr>
    {% endif %}
     {% if rfc.authors != [""] %}
     <tr>
     <td class="col-key">Authors</td>
      <td>
               <ul class="comma-list">
                 {% for author in rfc.authors %}
                 <li><a href="{{ gerrit_profile }}{{ author }}">{{ author }}</a></li>
                 {% endfor %}
               </ul>
       </td>
    </tr>
     {% endif %}
     {% if rfc.reviewers != [""] %}
     <tr>
     <td class="col-key">Reviewers</td>
      <td>
        <ul class="comma-list">
          {% for reviewer in rfc.reviewers %}
          <li><a href="{{ gerrit_profile }}{{ reviewer }}">{{ reviewer }}</a></li>
          {% endfor %}
        </ul>
      </td>
    </tr>
     {% endif %}
    {% if rfc.submitted != "" %}
    <tr>
      <td class="col-key">Date submitted (year-month-day)</td><td>{{ rfc.submitted }}</td>
    </tr>
    {% endif %}
    {% if rfc.reviewed != "" %}
    <tr>
      <td class="col-key">Date reviewed (year-month-day)</td><td>{{ rfc.reviewed }}</td>
    </tr>
    {% endif %}
  </tbody>
</table>

<div class="edit-buttons">
  <div class="edit-buttons-left">
    <p><img src="https://fonts.gstatic.com/s/i/googlematerialicons/edit/v6/googblue-24dp/1x/gm_edit_googblue_24dp.png" class="inline-icon" alt=""> <a href="{{ fuchsia_editor }}{{ rfcs_dir }}{{ rfc.file }}">Edit this RFC</a></p>
  </div>
  <div class="edit-buttons-right">
    <p><img src="https://fonts.gstatic.com/s/i/googlematerialicons/edit/v6/googblue-24dp/1x/gm_edit_googblue_24dp.png" class="inline-icon" alt=""> <a href="{{ fuchsia_editor }}{{ rfcs_dir }}{{ rfcs_metadata_file }}">Edit RFC metadata</a></p>
  </div>
</div>

<meta name="keywords" value="RFC{{ rfc.name|trim('RFC-')|int }}, RFC {{ rfc.name|trim('RFC-')|int }}" />
