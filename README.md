[![License](https://img.shields.io/badge/License-Apache_2.0-green.svg)](https://opensource.org/licenses/Apache-2.0)

# cogru
> Where the collaboration start!?

[![CI](https://github.com/Cogru/cogru/actions/workflows/test.yml/badge.svg)](https://github.com/Cogru/cogru/actions/workflows/test.yml)

## 💡 Rationale

Cogru aims to streamline pair programming, ensuring it operates seamlessly
without duplicating the features of other existing software, such as chat and
screen sharing. Our focus will be solely on enhancing the editing experience
and facilitating communication between workspaces from start to finish.

Here is a list of features I want Cogru to achieve:

- **Text Editing**, minimal lag as possible
- **Configurable**, allows you to release the power of the server's settings.
- **Interactable** commands that you can use to communicate with your co-workers!
- **Manageable**, take care of the logging, admin, and the workspace's permission.

## 🚧 Project status

The code in this repository is currently under active development, and may
therefore change substantially with each commit.

## 🔧 Usage

Run the server in the current working directory without requiring the password
to be entered.

```sh
$ cogru . --no-password
```

## 🪵 Client Implementation

- [ ] Emacs ([cogru.el][])
- [ ] Sublime Text ([cogru-sublime][])
- [ ] Vim
- [ ] VSCode
- [ ] Visual Studio
- [ ] Intellij

## 📁 Similar Projects

- [floobits][] (abandoned)
- [tandem][]
- [duckly][] (malware?)

## ⚜ License

Copyright 2024-present Cogru Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [`LICENSE`](./LICENSE) for details.


<!-- Links -->

[cogru.el]: https://github.com/Cogru/cogru.el
[cogru-sublime]: https://github.com/Cogru/cogru-sublime

[floobits]: https://floobits.com/
[tandem]: https://github.com/typeintandem/tandem
[duckly]: https://duckly.com/
