<!-- 
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
 -->
<aside class="key-point">
  <b>无环境权限</b>
  <p>Fuchsia 的<a href="/concepts/principles/secure.md">安全设计原则</a>之一是系统上的程序“无环境权限”。这意味着每个操作都必须限定在对象能力范围内，而不是根据更高级别的范围（例如用户身份或角色）授予访问权限。</p>

  <p>组件框架通过确保组件只能直接访问由其父组件显式路由的能力来支持这一原则。对通过环境提供的能力的访问由组件管理器代表组件进行调解。</p>
</aside>
