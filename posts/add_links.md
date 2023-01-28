```metadata
{
    "cover": "../app/assets/sources/links_cover.svg",
    "tag": "Help",
    "title": "如何申请友情链接"
}
```

Hii，欢迎来到我的站点，我很乐意跟你连接，你可以向我发起一个 PR 来进行友情链接的添加，你的网站将会被陈列在 [这里](/links)。

跟随以下几个步骤来将你的网站添加到 [友情链接](/links) 吧！

#### 1. Fork & Update links

Fork [zzhack](https://github.com/zzhack-stack/zzhack) 到你的 GitHub，修改 `/services/links_service/links.json` 添加你的网站信息。

```json
{
  "links": [
    {
      "name": "Busyops博客",
      "addr": "https://busyops.com/",
      "desc": "Hello Moon",
      "logo": "https://busyops.com/images/avatar.jpg"
    },
    {
      "name": "Clay 的技术博客",
      "addr": "https://www.techgrow.cn",
      "desc": "用进废退 | 艺不压身",
      "logo": "https://www.techgrow.cn/img/head.jpg"
    },
    {
      "name": "Christine的博客",
      "desc": "虽然我不够优秀，但我从未放弃过努力。",
      "logo": "https://christine-only.github.io/blog/logo.png",
      "addr": "https://christine-only.github.io/blog/"
    },
    {
      "name": "Forever丿顾北博客",
      "addr": "https://forevergubei.gitee.io/myblod/",
      "desc": "一个追寻大佬脚步的小白",
      "logo": "https://forevergubei.gitee.io/myblod/logo.png"
    }
    // {
    //     "name": "站点名称",
    //     "addr": "站点链接",
    //     "desc": "站点描述",
    //     "logo": "站点 logo"
    // }，
    // <- 加到这里 :D
  ]
}
```

#### 2. 提交修改

通过 GitHub 创建一个 PR 合并到 `main`，等待 merge 后就能在 [友情链接](/links) 看到你的站点了。

#### Final

如果你嫌上述步骤太麻烦也不要紧，将你的网站信息通过 [Email](mailto:mist.zzh@gmail.com) 发给我，我会在空闲的时候处理 :D。
