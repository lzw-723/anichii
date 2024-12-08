import {useEffect, useState} from 'preact/hooks'
import "bootstrap/dist/js/bootstrap.js"
import "halfmoon/css/halfmoon.css"
import "bootstrap-icons/font/bootstrap-icons.css";
import './app.css'
import {add_anime, delete_anime, fetch_anime} from "./api.js";
import {SettingView} from "./views/SettingView.jsx";
import {AnimeModal} from "./components/AnimeModal.jsx";
import {EpisodesModal} from "./components/EpisodesModal.jsx";
import Counter from "./components/Counter.jsx";

export function App() {
    const [anime, setAnime] = useState([])
    const [curAnime, setCurAnime] = useState({})
    const [viewAnime, setViewAnime] = useState({})

    const [showGrid, setShowGrid] = useState(true)

    function listAnime() {
        fetch_anime().then(res => {
            console.log(res);
            setAnime(res);
        })
    }

    function addAnime() {
        add_anime({
            title: curAnime.title || '', sub: curAnime.sub || '', rss: curAnime.rss
        }).then(r => {
            console.log("add anime success");
            listAnime();
        });
    }

    function deleteAnime(id) {
        delete_anime(id).then(r => {
            console.log("delete anime success")
            listAnime()
        });
    }

    const animeItems = anime.map((anime) => <tr>
        <td>{anime.title}</td>
        <td>{anime.sub}</td>
        <td>{anime.rss}</td>
    </tr>);
    const animeList = anime.map((anime) =>

        <div className="card" style="width: 15rem;">
            <div className="card-body">
                <h5 className="card-title">{anime.title}</h5>
                <div className="card-text">

                    <p>{anime.sub}</p>
                    <a href="#eps-modal" type="button" className="btn btn-secondary position-relative"
                       data-bs-toggle="modal" onClick={() => setViewAnime(anime)}>
                        <i className="bi bi-files"></i>剧集
                        <span
                            className="badge position-absolute top-0 start-100 translate-middle text-bg-primary">{anime.episodes.length}</span>
                    </a>
                </div>
                <div className="btn-group" role="group" aria-label="Basic example">
                    <a href="#view-modal" role="button" class="btn btn-secondary" data-bs-toggle="modal"
                       onClick={() => setViewAnime(anime)}>查看</a>
                    <a href="#" class="btn btn-secondary" onClick={() => deleteAnime(anime.id)}>删除</a>
                </div>
            </div>
        </div>)

    useEffect(() => {
        listAnime();
        setInterval(() => {
            console.log("refresh");
            listAnime();
        }, 5000);
    }, []);

    return (<>
        <nav className="navbar navbar-expand-md"
             style="background-color: var(--bs-content-bg); border-bottom: var(--bs-border-width) solid var(--bs-content-border-color);">
            <div className="container-fluid">
                <a className="navbar-brand" href="#">
                    {/*<img src="..." alt="Logo" width="24" height="24" className="d-inline-block align-text-top"/>*/}
                    AniChii
                </a>
                <button className="navbar-toggler" type="button" data-bs-toggle="collapse"
                        data-bs-target="#navbar-collapse-2" aria-controls="navbar-collapse-2" aria-expanded="false"
                        aria-label="Toggle navigation">
                    <span className="navbar-toggler-icon"></span>
                </button>
                <div className="collapse navbar-collapse" id="navbar-collapse-2">
                    <ul className="navbar-nav ms-auto">
                        <li className="nav-item">
                            <a className="nav-link active" data-bs-toggle="offcanvas" href="#setting-offcanvas">
                                <i class="bi bi-gear-fill"></i>设置</a>
                        </li>
                        {/*<li className="nav-item">*/}
                        {/*    <a className="nav-link" href="#">*/}
                        {/*        <i class="bi bi-box-arrow-right"></i>退出</a>*/}
                        {/*</li>*/}
                        <li className="nav-item">
                            <a className="nav-link" href="#about-modal" data-bs-toggle="modal">
                                <i class="bi bi-info-circle"></i>关于</a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>

        <div className="modal fade" id="about-modal" tabIndex="-1" aria-hidden="true">
            <div className="modal-dialog modal-dialog-centered">
                <div className="modal-content">
                    <div className="modal-header">
                        <h1 className="modal-title fs-5" id="modal-title-2">关于AniChii</h1>
                        <button type="button" className="btn-close" data-bs-dismiss="modal"
                                aria-label="Close"></button>
                    </div>
                    <div className="modal-body">
                        <p>
                            AniChii是一个基于RSS的自动番剧下载工具，依赖qBittorrent。<br/>
                            如果有什么问题或者建议，欢迎提issue或者PR。
                        </p>
                        <div className="hstack gap-2 my-2">
                            <a href="https://github.com/lzw-723/anichii" target="_blank" className="btn btn-secondary">
                                <i className="bi bi-github me-1"></i>
                                GitHub
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <SettingView/>

        <div className="container">


            <div className="hstack gap-2 my-2">
                <div className="btn-group" role="group" aria-label="Basic example">
                    <button className="btn btn-secondary" onClick={() => listAnime()}>
                        <i className="bi bi-arrow-clockwise"></i>刷新
                    </button>
                    <button type="button" className="btn btn-primary" data-bs-toggle="modal"
                            data-bs-target="#example-modal-2">
                        <i className="bi bi-plus"></i>
                        添加
                    </button>
                </div>
                <div className="vr ms-auto"></div>
                <div className="dropdown">
                    <a href="#" className="btn btn-link dropdown-toggle" role="button" data-bs-toggle="dropdown"
                       aria-expanded="false">
                        {showGrid ? '网格' : '列表'}
                    </a>
                    <ul className="dropdown-menu">
                        <li><h6 className="dropdown-header">显示方式</h6></li>
                        <li>
                            <button type="button" className="dropdown-item" aria-current="true"
                                    onClick={() => setShowGrid(true)}>网格
                            </button>
                        </li>
                        <li>
                            <button type="button" className="dropdown-item"
                                    onClick={() => setShowGrid(false)}
                            >列表
                            </button>
                        </li>
                    </ul>
                </div>
            </div>


            {showGrid ? <div className="d-flex flex-wrap gap-2">
                {animeList}
            </div> : <table className="table">
                <thead>
                <tr>
                    <th scope="col">名称</th>
                    <th scope="col">字幕组</th>
                    <th scope="col">RSS</th>
                </tr>
                </thead>
                <tbody>
                {animeItems}
                </tbody>
            </table>}

            <AnimeModal id={viewAnime.id}/>
            <EpisodesModal id={viewAnime.id}/>


            <div className="modal fade" id="example-modal-2" data-bs-backdrop="static" data-bs-keyboard="false"
                 tabIndex="-1" aria-labelledby="modal-title-2" aria-hidden="true">
                <div className="modal-dialog">
                    <div className="modal-content">
                        <div className="modal-header">
                            <h1 class="modal-title fs-5" id="modal-title-2">添加订阅</h1>
                            <button type="button" className="btn-close" data-bs-dismiss="modal"
                                    aria-label="Close"></button>
                        </div>
                        <div className="modal-body">
                            <div>
                                <label htmlFor="example-textarea-1"
                                       className="form-label">RSS</label>
                                <textarea className="form-control" id="example-textarea-1" rows="3"
                                          type="url"
                                          placeholder="RSS链接" onChange={(e) => {
                                    let anime = {...curAnime};
                                    anime.rss = e.target.value;
                                    setCurAnime(anime)
                                }}></textarea>
                            </div>
                            <br/>
                            <div>
                                <label htmlFor="example-textarea-1"
                                       className="form-label">季度：</label>
                                <Counter onChange={(c) => setCurAnime({...curAnime, season: c})}/>
                            </div>

                            <button type="button" className="btn btn-link" data-bs-toggle="collapse"
                                    data-bs-target="#collapse-example-1" aria-expanded="true"
                                    aria-controls="collapse-example-1">
                                更多<i className="fa-light fa-angle-down"></i>
                            </button>
                            <div className="collapse show" id="collapse-example-1">
                                <div className="mb-3">
                                    <label htmlFor="example-input-1" className="form-label">名称</label>
                                    <input className="form-control" id="example-input-1"
                                           placeholder="番剧名称" onChange={(e) => {
                                        let anime = {...curAnime};
                                        anime.title = e.target.value;
                                        setCurAnime(anime)
                                    }}/>
                                </div>
                                <div className="mb-3">
                                    <label htmlFor="example-input-1" className="form-label">字幕组</label>
                                    <input className="form-control" id="example-input-1"
                                           placeholder="字幕组" onChange={(e) => {
                                        let anime = {...curAnime};
                                        anime.sub = e.target.value;
                                        setCurAnime(anime)
                                    }}/>
                                </div>
                            </div>

                        </div>
                        <div className="modal-footer">
                            <button type="button" className="btn btn-secondary" data-bs-dismiss="modal">取消
                            </button>
                            <button type="button" className="btn btn-primary" data-bs-dismiss="modal"
                                    disabled={!curAnime.rss}
                                    onClick={addAnime}>添加
                            </button>
                        </div>
                    </div>
                </div>
            </div>

        </div>

    </>)
}
