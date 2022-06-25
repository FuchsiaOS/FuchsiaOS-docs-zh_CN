  <tr class = "sys">
    <td><p>{{ sys.name }}<a name="{{ sys.name|replace(" ", "-")|replace("(", "")|replace(")", "")|lower() }}"></a></p><h3 class="add-link" style="display:none">{{ sys.name }}</h3></td>
    <td>
      <table class = "nested responsive">
        <colgroup>
        <col width="10%">
      </colgroup>
        <tbody class="list">
          {%- if sys.description|length %}
          <tr>
            <td>Description</td>
            <td>{{ sys.description }}</td>
          </tr>
          {%- endif %}
          {%- if sys.manufacturer_link|length %}
          <tr>
            <td>Manufacturer link</td>
            <td><a class= "external" href="{{ sys.manufacturer_link }}">{{ sys.name }}</a></td>
          </tr>
          {%- endif %}
          {%- if sys.architecture|length %}
          <tr>
            <td>Architecture</td>
            <td>{{ sys.architecture }}</td>
          </tr>
          {%- endif %}
          {%- if sys.RAM|length %}
          <tr>
            <td>RAM</td>
            <td>{{ sys.RAM }}</td>
          </tr>
          {%- endif %}
          {%- if sys.storage|length %}
          <tr>
            <td>Storage</td>
            <td>{{ sys.storage }}</td>
          </tr>
          {%- endif %}
          {%- if sys.board_driver_location|length %}
          <tr>
            <td>Board driver path</td>
            <td><a href="{{ cs_url }}{{ sys.board_driver_location}}"><code>/{{ sys.board_driver_location }}</code></a></td>
          </tr>
          {%- endif %}
          {%- if sys.name|length %}
            {%- set found=false %}
            {%- set printed=false %}
            {%- for driver in drivers.drivers_documentation %}
              {%- if driver.supported_system_configurations %}
                {%- for sysd in driver.supported_system_configurations %}
                  {%- if sysd == sys.name %}
                    {%- set found=true %}
                    {% if found and printed == false %}
                    <tr>
                      <td>Drivers</td>
                      <td>
                        <ul class="driver-sys-list-outer">
                    {%- set printed=true %}
                    {%- endif %}
                    <li>
                    {%- if driver.areas %}
                    <ul class="driver-sys-list-inner">
                    {%- for area in driver.areas %}
                    <li>{{ area }}</li>
                    {%- endfor %}
                    </ul>
                    <a href="{{driver_page }}{{ driver.name }}">{{ driver.name }}</a></li>
                    {%- endif %}
                          {%- endif %}
                 {%- endfor %}
              {%- endif %}
            {%- endfor %}
              </ul>
            </td>
          </tr>
          {%- endif %}
        </tbody>
      </table>
    </td>
  </tr>