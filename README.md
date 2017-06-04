# V2EX Sign
　　一个帮你自动在 [v2ex.com](https://www.v2ex.com/) 上领取每日奖励的小脚本。

## Usage
　　直接执行 `v2ex-sign`，会加载可执行文件当前目录下的 cookie 文件作为登录 cookie。也可以使用 `-c <cookie>` 选项指定 cookie 文件的位置。

　　cookie 文件应该只有一行，对应 Request Header 中的 Cookie 字段的值。

## 定时任务
　　推荐使用 systemd timer 来定时执行签到脚本。

　　在 `/etc/systemd/system` 目录下建立下列文件：

- v2ex-sign.service
```
[Unit]
Description=v2ex sign
After=network.service

[Service]
Type=oneshot
ExecStart=/home/sbw/v2ex-sign -c /home/sbw/cookie
```

- v2ex-sign.timer
```
[Unit]
Description=v2ex sign

[Timer]
OnCalendar=*-*-* 05:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

　　刷新配置并开启定时任务
```
# systemctl daemon-reload
# systemctl enable v2ex-sign.timer
# systemctl start v2ex-sign.timer
```

　　这样就可以在每天的 `5:00 PM` 自动执行签到程序。可以使用 `journalctl -u v2ex-sign` 查看执行结果，或者使用 `systemctl list-timers` 查看定时任务执行状态。