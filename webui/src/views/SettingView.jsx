import {useEffect, useState} from "preact/hooks";
import {fetch_setting, test_qb, update_setting} from "../api.js";

export function SettingView() {

    const [setting, setSetting] = useState({})

    const [testing, setTesting] = useState(false)
    const [testFailed, setTestFailed] = useState(null)

    function get_setting() {
        fetch_setting().then(res => {
            console.log(res);
            setSetting(res);
        })
    }

    function save_setting() {
        update_setting(setting).then(r => {
            console.log(r);
        });
    }

    function test() {
        setTesting(true)
        test_qb({
            url: setting.qb_url, username: setting.qb_username, password: setting.qb_password,
        }).then(res => {
            console.log(res);
            setTestFailed(!res)
        }).finally(() => {
            setTesting(false)
        })
    }

    useEffect(() => {
        get_setting()
    }, []);

    return (<>
        <div className="offcanvas offcanvas-start" tabIndex="-1" id="setting-offcanvas"
             aria-labelledby="offcanvas-title-1">
            <div className="offcanvas-header">
                <h5 className="offcanvas-title" id="offcanvas-title-1">设置</h5>
                <button type="button" className="btn-close" data-bs-dismiss="offcanvas" aria-label="Close"
                        onClick={get_setting}></button>
            </div>
            <div className="offcanvas-body">
                <ul className="nav nav-tabs" id="js-tabs-1" role="tablist">
                    <li className="nav-item" role="presentation">
                        <button className="nav-link active" id="home-tab" data-bs-toggle="tab"
                                data-bs-target="#home-tab-pane" type="button" role="tab" aria-controls="home-tab-pane"
                                aria-selected="true">下载器
                        </button>
                    </li>
                    <li className="nav-item" role="presentation">
                        <button className="nav-link" id="account-tab" data-bs-toggle="tab"
                                data-bs-target="#account-tab-pane" type="button" role="tab"
                                aria-controls="account-tab-pane" aria-selected="false">连接设置
                        </button>
                    </li>
                    <li className="nav-item" role="presentation">
                        <button className="nav-link disabled" id="disabled-tab" data-bs-toggle="tab"
                                data-bs-target="#disabled-tab-pane" type="button" role="tab"
                                aria-controls="disabled-tab-pane" aria-selected="false" disabled>其他
                        </button>
                    </li>
                </ul>
                <div className="tab-content" id="js-tabs-content-1">
                    <div className="tab-pane fade show active" id="home-tab-pane" role="tabpanel"
                         aria-labelledby="home-tab" tabIndex="0">
                        <div class="my-2 text-body-secondary">
                            <p>设置你的下载器，当前仅支持qBittorrent。</p>
                            <p>在保存前可以点击测试按钮测试可用性。</p>
                        </div>
                        <div className="mb-3">
                            <label htmlFor="example-input-1" className="form-label">qB地址</label>
                            <input type="url" className="form-control" id="example-input-1"
                                   placeholder="url" defaultValue={setting.qb_url}
                                   onChange={e => setSetting({...setting, qb_url: e.target.value})}
                            />
                        </div>
                        <div className="mb-3">
                            <label htmlFor="example-input-1" className="form-label">用户名</label>
                            <input type="username" className="form-control" id="example-input-1"
                                   placeholder="admin" defaultValue={setting.qb_username}
                                   onChange={e => setSetting({...setting, qb_username: e.target.value})}
                            />
                        </div>
                        <div className="mb-3">
                            <label htmlFor="example-input-1" className="form-label">密码</label>
                            <input type="password" className="form-control" id="example-input-1"
                                   placeholder="密码" defaultValue={setting.qb_password}
                                   onChange={e => setSetting({...setting, qb_password: e.target.value})}
                            />
                        </div>
                        <div className="mb-3">
                            <label htmlFor="example-input-1" className="form-label">保存位置</label>
                            <input type="path" className="form-control" id="example-input-1"
                                   placeholder="例如/downloads" defaultValue={setting.qb_save_path}
                                   onChange={e => setSetting({...setting, qb_save_path: e.target.value})}
                            />
                        </div>
                        <button
                            className={"btn " + (testFailed === null ? "" : (testFailed ? "btn-danger" : "btn-success"))}
                            type="button" disabled={testing} onClick={test}>
                    <span className="spinner-border spinner-border-sm" role="status" aria-hidden="true"
                          hidden={!testing}></span>
                            测试
                        </button>
                        <button type="button" className="btn btn-primary" onClick={save_setting}>保存</button>
                    </div>

                    <div className="tab-pane fade" id="account-tab-pane" role="tabpanel" aria-labelledby="account-tab"
                         tabIndex="0">

                        <label className="form-label">网络代理</label>
                        <div className="form-check">
                            <input className="form-check-input" type="radio" value="backend" name="radio-group-1"
                                   id="example-radio-1" checked/>
                            <label className="form-check-label" htmlFor="example-radio-1">
                                不使用代理
                            </label>
                        </div>
                        <div className="form-check">
                            <input className="form-check-input" type="radio" value="frontend" name="radio-group-1"
                                   id="example-radio-2" disabled/>
                            <label className="form-check-label" htmlFor="example-radio-2">
                                使用系统代理
                            </label>
                        </div>
                        <div className="form-check">
                            <input className="form-check-input" type="radio" value="fullstack" name="radio-group-1"
                                   id="example-radio-3" disabled/>
                            <label className="form-check-label" htmlFor="example-radio-3">
                                手动配置代理
                            </label>
                            <div className="input-group mb-3 mt-1">
                                <span className="input-group-text" id="add-on-1">http代理</span>
                                <input type="text" className="form-control" placeholder="Username" aria-label="Username"
                                       aria-describedby="add-on-1"/>
                            </div>
                            <div className="input-group mb-3">
                                <span className="input-group-text" id="add-on-1">https代理</span>
                                <input type="text" className="form-control" placeholder="Username" aria-label="Username"
                                       aria-describedby="add-on-1"/>
                            </div>
                        </div>

                    </div>
                    <div className="tab-pane fade" id="disabled-tab-pane" role="tabpanel" aria-labelledby="disabled-tab"
                         tabIndex="0">...
                    </div>
                </div>


            </div>
        </div>
    </>)
}
