#[macro_use]
extern crate lazy_static;
extern crate tera;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;

use convert::Data;

mod convert;

lazy_static! {
    pub static ref TERA: Tera = Tera::default();
}

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(WEBUI)
}

async fn convert(body: web::Json<Data>) -> impl Responder {
    let data = body.0;
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(data.convert())
}

async fn check(body: web::Json<Data>) -> impl Responder {
    let data = body.0;
    HttpResponse::Ok().body(data.check().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/convert", web::post().to(convert))
            .route("/check", web::post().to(check))
    })
    .bind("127.0.0.1:3993")?
    .run()
    .await
}

pub const WEBUI: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>cffc</title>
    <link rel="stylesheet" href="https://unpkg.com/element-ui/lib/theme-chalk/index.css">
    <style>
        .container {
            display: flex;
            flex-direction: row;
            justify-content: space-around;
            margin: 1% 2%;
        }

        .left {
            width: 45%;
        }

        .middle {
            width: 10%;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }

        .right {
            width: 45%;
        }

        .el-textarea > textarea {
            height: 600px;
            resize: none;
        }
    </style>
</head>
<body>
<div id="cffc" class="container">
    <div class="left">
        <el-select v-model="from" clearable placeholder="请选择">
            <el-option
                    v-for="item in fts"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value">
            </el-option>
        </el-select>

        <el-form>
            <el-form-item>
                <el-input type="textarea" v-model="from_text"></el-input>
            </el-form-item>
            <el-form-item>
                <el-button type="success" @click="check('left')">格式校验</el-button>
                <el-button type="primary" @click="copy('from')">复制</el-button>
            </el-form-item>
        </el-form>
    </div>
    <div class="middle">
        <el-button icon="el-icon-d-arrow-right" @click="convert('right')"></el-button>
        <el-divider></el-divider>
        <el-button icon="el-icon-d-arrow-left" @click="convert('left')"></el-button>
    </div>
    <div class="right">
        <el-select v-model="to" clearable placeholder="请选择">
            <el-option
                    v-for="item in fts"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value">
            </el-option>
        </el-select>
        <el-form>
            <el-form-item>
                <el-input type="textarea" v-model="to_text"></el-input>
            </el-form-item>
            <el-form-item>
                <el-button type="success" @click="check('right')">格式校验</el-button>
                <el-button type="primary" @click="copy('to')">复制</el-button>
            </el-form-item>
        </el-form>
    </div>
</div>

<script src="https://cdn.jsdelivr.net/npm/vue"></script>
<script src="https://unpkg.com/element-ui/lib/index.js"></script>
<script src="https://cdn.bootcdn.net/ajax/libs/axios/0.21.1/axios.min.js"></script>
<script src="https://cdn.bootcdn.net/ajax/libs/vue-clipboard2/0.3.1/vue-clipboard.js"></script>
<script>
    var cffc = new Vue({
        el: '#cffc',
        data: () => {
            return {
                fts: [{
                    value: 'JSON',
                    label: 'JSON'
                }, {
                    value: 'YAML',
                    label: 'YAML'
                }, {
                    value: 'TOML',
                    label: 'TOML'
                }],
                from: '',
                to: '',
                from_text: '',
                to_text: '',
                loading: false,
            }
        },
        methods: {
            convert(lr) {
                this.loading = true
                if (lr === 'right') {
                    axios.post("/convert", {
                        from: this.from,
                        to: this.to,
                        text: this.from_text,
                    }).then(resp => {
                        if (this.to === 'JSON' || this.to === 'json') {
                            this.to_text = JSON.stringify(resp.data);
                        } else {
                            this.to_text = resp.data;
                        }
                        this.loading = false
                    }).catch(resp => {
                        this.loading = false
                        this.$notify.error({
                            title: '转换失败',
                            message: resp,
                            duration: 2000
                        });
                    })
                } else if (lr === 'left') {
                    axios.post("/convert", {
                        from: this.to,
                        to: this.from,
                        text: this.to_text,
                    }).then(resp => {
                        if (this.from === 'JSON' || this.from === 'json') {
                            this.from_text = JSON.stringify(resp.data);
                        } else {
                            this.from_text = resp.data;
                        }
                        this.loading = false
                    }).catch(resp => {
                        this.loading = false
                        this.$notify.error({
                            title: '错误',
                            message: resp,
                            duration: 2000
                        });
                    })
                }
            },
            check(lr) {
                if (lr === 'left') {
                    axios.post("/check", {
                        from: this.from,
                        text: this.from_text,
                    }).then(resp => {
                        if (resp.data) {
                            this.$message({
                                showClose: true,
                                message: '正确的 ' + this.from,
                                type: 'success'
                            });
                        } else {
                            this.$message({
                                showClose: true,
                                message: '不正确的 ' + this.from,
                                type: 'error'
                            });
                        }
                    }).catch(e => {
                        this.$message({
                            showClose: true,
                            message: e,
                            type: 'error'
                        });
                    })
                } else if (lr === 'right') {
                    axios.post("/check", {
                        from: this.to,
                        text: this.to_text,
                    }).then(resp => {
                        if (resp.data) {
                            this.$message({
                                showClose: true,
                                message: '正确的 ' + this.to,
                                type: 'success'
                            });
                        } else {
                            this.$message({
                                showClose: true,
                                message: '不正确的 ' + this.to,
                                type: 'error'
                            });
                        }
                    }).catch(e => {
                        this.$message({
                            showClose: true,
                            message: e,
                            type: 'error'
                        });
                    })
                }
            },
            copy(ft) {
                let text = "";
                if (ft === 'from') {
                    text = this.from_text;
                } else if (ft === 'to') {
                    text = this.to_text;
                }
                this.$copyText(text).then(e => {
                    this.$message({
                        showClose: true,
                        message: '复制成功',
                        type: 'success'
                    });
                }, e => {
                    this.$message({
                        showClose: true,
                        message: '复制失败',
                        type: 'error'
                    });
                })
            },
        }
    });
</script>
</body>
</html>
"#;
