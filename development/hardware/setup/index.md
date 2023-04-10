# Device setup guides index

The table below is a comprehensive index of all guides related to getting
Fuchsia running on a device. Use the dropdown menu next to the table to filter
the results. See [Descriptions](#descriptions) for an explanation of each dropdown menu.

{%- set metadata | yamlloads %}
{% include "docs/development/hardware/setup/_metadata.yaml" %}
{%- endset %}

<section>
  <devsite-select id="types">
    <select>
      <option>Type</option>
      {% if metadata.types %}
        {% for type in metadata.types %}
          <option>{{type}}</option>
        {% endfor %}
      {% endif %}
    </select>
  </devsite-select>
  <devsite-select id="products">
    <select>
      <option>Product</option>
      {% if metadata.products %}
        {% for product in metadata.products %}
          <option>{{product}}</option>
        {% endfor %}
      {% endif %}
    </select>
  </devsite-select>
  <devsite-select id="boards">
    <select>
      <option>Board</option>
      {% if metadata.boards %}
        {% for board in metadata.boards %}
          <option>{{board}}</option>
        {% endfor %}
      {% endif %}
    </select>
  </devsite-select>
  <devsite-select id="methods">
    <select>
      <option>Method</option>
      {% if metadata.methods %}
        {% for method in metadata.methods %}
          <option>{{method}}</option>
        {% endfor %}
      {% endif %}
    </select>
  </devsite-select>
  <devsite-select id="hosts">
    <select>
      <option>Host</option>
      {% if metadata.hosts %}
        {% for host in metadata.hosts %}
          <option>{{host}}</option>
        {% endfor %}
      {% endif %}
    </select>
  </devsite-select>
  <devsite-filter select-el-container-id="types products boards methods hosts">
    <table>
      <thead>
        <tr>
          {% if metadata.columns %}
            {% for column in metadata.columns %}
              <th>{{column}}</th>
            {% endfor %}
          {% endif %}
        </tr>
      </thead>
      <tbody class="list">
        {% if metadata.guides %}
          {% for guide in metadata.guides %}
            {% if guide.type and guide.product and guide.board and guide.method and guide.host and guide.url and guide.title %}
              <tr>
                <td>{{guide.type}}</td>
                <td>{{guide.product}}</td>
                <td>{{guide.board}}</td>
                <td>{{guide.method}}</td>
                <td>{{guide.host}}</td>
                <td><a href="{{guide.url}}">{{guide.title}}</a></td>
              </tr>
            {% endif %}
          {% endfor %}
        {% endif %}
      </tbody>
    </table>
  </devsite-filter>
</section>

## Descriptions {#descriptions}

{% if metadata.descriptions %}
  <ul>
    {% for key, description in metadata.descriptions.items() %}
      <li>{{description}}</li>
    {% endfor %}
  </ul>
{% endif %}
