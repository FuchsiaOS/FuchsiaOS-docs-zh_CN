<!-- ## Hosting and serving packages -->
## 包托管和服务

<!-- Packages are hosted in **repositories** based on
[The Update Framework](https://theupdateframework.io/){:.external} (TUF).
This framework is a specification designed to enable secure delivery of software
updates. TUF repositories secure updates through signed metadata attached to
records that are verifiable against known trusted public and private keys.
This means that **any HTTP server can serve a TUF repository** without the need
for transport-level security, **_including a developer's workstation!_** -->
包托管在位于[更新框架](https://theupdateframework.io/){:.external} (TUF)的**仓库**中。该框架是一种规范，旨在实现软件更新的安全交付。 TUF 仓库通过附加到记录的签名元数据来保护更新，这些记录可根据已知的可信公钥和私钥进行验证。

<aside class="key-point">
<!-- Developer tools such as <code>ffx</code> host a simple HTTP server locally
serving a static tree of files formatted as a TUF repository. -->
<code>ffx</code> 等开发人员工具在本地托管一个简单的 HTTP 服务器，为格式化为 TUF 仓库的静态文件树提供服务。
</aside>

<!-- Packages within a repository are identified by a URL with the
`fuchsia-pkg` scheme: -->
仓库中的包通过 `fuchsia-pkg` 格式的 URL 标识：

```none
fuchsia-pkg://{{ '<var>' }}repo-hostname{{ '</var>' }}/{{ '<var>' }}pkg-name{{ '</var>' }}#{{ '<var>' }}resource-path{{ '</var>' }}
```

<!-- * `repo-hostname`: Hostname of a trusted package repository, such as `fuchsia.com`.
* `pkg-name`: Unique identifier for the package in this repository.
* `resource-path`: Resource contained within the package, such as a component
  manifest. -->
* `repo-hostname`：受信任的软件包仓库的主机名，比如，`fuchsia.com`。
* `pkg-name`：本仓库中包的唯一标识。
* `resource-path`：包中包含的资源，比如，组件清单。

<!-- ![Diagram showing how packages are resolved from a TUF repository and cached
locally on the device.] -->
![如何解析 TUF 仓库中的包并在设备本地缓存的示意图]
(/get-started/images/intro/package-resolver.png){: width="751"}

<!-- Requests for software on a Fuchsia device are handled by the
**package resolver**. The package resolver determines if the system already has
the package cached locally. If not, the resolver fetches the meta.far from the
repository and updates the necessary content BLOBs. -->
Fuchsia 设备上的软件请求由**package resolver**处理。包解析器确定系统是否已经在本地缓存了包。如果没有，解析器从存储库中获取 meta.far 并更新必要的内容 BLOB。
