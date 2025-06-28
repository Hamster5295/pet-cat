# Pet-Cat

Pet-Cat 是基于 [Kovi](https://kovi.thricecola.com/) 框架的自动撸猫插件，当群组中有人发猫猫时，自动回复指定的图片 \(最佳实践是一个撸猫的表情包\)。  

## 安装

1. 根据[教程](https://kovi.thricecola.com/start/fast.html)创建一个 Kovi 工程
2. 在项目根目录运行
```bash
cargo add kovi-plugin-pet-cat
```

3. 在 `build_bot!` 宏中传入插件
```rust
let bot = build_bot!(kovi-plugin-pet-cat /* 和其他你正在使用的插件，用 , 分割 */ );
```

## 配置

Copycat 可以通过 `toml` 文件进行配置。如果配置文件不存在，则使用默认配置。  

```toml
# 带有视觉模态的 LLM
api_url = "https://someapi/v1/chat/completions"
api_key = "Some-API-Key"
model = "some-vision-model"

# [选填]
# 用于判定猫猫的 prompt
# 插件内置了另一个 prompt 用于提示 LLM 输出“是”或“否”，此处可以不提及
# 如果没有这一项，则使用默认的 prompt，（可能提高误判概率）
prompt = "请辨别这张图片是否包含一只真实的猫咪，而非卡通猫咪或表情包。如果这张图片包含**修图软件添加的文字**，请回答'否'。"

# 回复的图片，考虑版权问题，此处不提供，可以选一个喜欢的
# 图片路径为可执行文件同级的 data/kovi-plugin-pet-cat/<下方提供的文件名>
pet_cat_img = "cat.gif"

# [选填]
# 在指定群聊中启用复读。如果没有此项，则在所有群聊中启用复读。
allow_groups = [123456789]

```  

配置文件应放置于编译后与可执行文件同级的 `data/kovi-plugin-pet-cat/config.toml` 中。