<aside class="key-point">
  <b>No ambient authority</b>
  <p>One of Fuchsia's <a href="/concepts/principles/secure.md">security
  design principles</a> is "no ambient authority" for programs on the system.
  This means that every operation must be scoped to an object capability rather
  than granting access based on a higher-level scope such as user identity or
  role.</p>

  <p>The component framework upholds this principle by ensuring that components
  only have direct access to capabilities explicitly routed by their parent.
  Access to capabilities provisioned through environments is mediated by
  Component Manager on the component's behalf.</p>
</aside>
