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

pet_cat_img = "cat.gif"     # 一个撸猫的表情包，放置在与配置文件同级的位置
allow_groups = []           # 群组白名单

# 条件们，可以有多个条件，同时满足才会撸猫
# 以下给出一个我自己使用的样例
[[conditions]]
name = "is_cat"             # 条件的名称，会显示在日志里，方便 Debug

# 条件的 Prompt，要求模型回答指定字符串
prompt = "你是一个猫咪图片鉴别大师，请辨别这张图片是否包含一只真实的猫咪，而非卡通角色。如果这张图片展示的是一只真实的猫咪，请回答'是'；如果这张图片是卡通猫咪，或者不包含猫咪，请回答'否'。不要回答其他内容，不要解释原因。"
prediction = "是"   # 如果模型回答了这个字符串，则判定条件成立，可以发猫猫

[[conditions]]
name = "no_text"
prompt = "你是一个文字识别专家，请判断这张图片的顶部和底部是否有文字。如果顶部和底部含有明显的文字，请回答'是'；如果没有，请回答'否'。不要回答其他内容，不要解释原因。"
prediction = "否"

[[conditions]]
name = "no_modification"
prompt = "你是一个图像鉴别大师，请判断这张图片是否经过后期修图技术修改。如果发现有图像修改痕迹，请回答'是'；如果这张图片没有任何修图痕迹，请回答'否'。不要回答其他内容，不要解释原因。"
prediction = "否"


```  

配置文件应放置于编译后与可执行文件同级的 `data/kovi-plugin-pet-cat/config.toml` 中。