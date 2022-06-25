  <tr class = "driver">
    {%- if driver.name %}
    <td><p>{{ driver.name }}<a name="{{ driver.name|replace(" ", "-")|replace("(", "")|replace(")", "")|lower() }}"></a></p><h3 class="add-link" style="display:none">{{ driver.name }}</h3></td>
    {% endif %}
    <td>
      <table class = "nested responsive">
        <colgroup>
        <col width="10%">
      </colgroup>
        <tbody class="list">
          {%- if driver.short_description %}
          <tr>
            <td title="A short description, no more than 80 characters.">
            Description {{info_icon}}</td>
            <td>{{ driver.short_description }}</td>
          </tr>
          {%- endif %}
          {%- if driver.manufacturer %}
          <tr>
            <td title="Manufacturer of the hardware the driver applies to.">
            Manufacturer {{info_icon}}</td>
            <td>{{ driver.manufacturer |capitalize() }}</td>
          </tr>
          {%- endif %}
          {%- if driver.families %}
          <tr>
            <td title="Families of hardware the driver applies to. It could be
              &quot;generic&quot; if the driver applies to wide variety of
              hardware, for instance implemented based on a standard
              specification or API.">Families {{info_icon}}</td>
            <td>
              <ul class="comma-list">
                {%- for fam in driver.families %}
                <li>{{ fam }}</li>
                {%- endfor %}
              </ul>
            </td>
          </tr>
          {%- endif %}
          {%- if driver.areas %}
          <tr>
            <td title="The general Fuchsia system areas this driver applies to.">
            Areas {{info_icon}}</td>
            <td>
              <ul class="comma-list">
                {%- for area in driver.areas %}
                <!-- area-{{ area }} -->
                <li>{{ area }}</li>
                {%- endfor %}
              </ul>
            </td>
          </tr>
          {%- endif %}
          {%- if driver.models %}
          <tr>
            <td title="Models the driver applies to. It could be
             &quot;generic&quot; if the driver applies to wide variety of
             hardware, for instance implemented based on a standard
            specification or API">Models {{info_icon}}</td>
            <td>
              <ul class="comma-list">
                {%- for mod in driver.models %}
                <li>{{ mod }}</li>
                {%- endfor %}
              </ul>
            </td>
          </tr>
          {%- endif %}
          {%- if driver.supported_system_configurations %}
          <tr>
            <td title="Supported Fuchsia system configurations for the driver.">
              Supported system configurations {{info_icon}}</td>
            <td>
              <ul class="comma-list">
                {%- for sys in driver.supported_system_configurations %}
                <li><a href="{{sys_config_page }}{{ sys|lower }}">{{ sys }}</a></li>
                {%- endfor %}
              </ul>
            </td>
          </tr>
          {%- endif %}
          {%- if driver.path %}
          <tr>
            <td title="The path in the Fuchsia source tree where the driver
            source code is currently located.">Path {{info_icon}}</td>
          {%- if driver.path[0] != '/' %}
            <td><a href="{{ cs_url }}{{ driver.path }}"><code>//{{ driver.path }}</code></a></td>
          {%- elif driver.path[0] == '/' and driver.path[1] != '/' %}
            <td><a href="{{ cs_url }}{{ driver.path[1:] }}"><code>/{{ driver.path }}</code></a></td>
          {%- elif driver.path[0] == '/' and driver.path[1] == '/' %}
            <td><a href="{{ cs_url }}{{ driver.path[2:] }}"><code>{{ driver.path }}</code></a></td>
          {%- else %}
            <td><a href="{{ cs_url }}{{ driver.path }}"><code>{{ driver.path }}</code></a></td>
          {%- endif %}
          </tr>
          {%- endif %}
        </tbody>
      </table>
    </td>
  </tr>