import {useEffect, useState} from "preact/hooks";
import {add_anime, add_keyword, delete_keyword, fetch_anime_by_id, fetch_keywords} from "../api.js";


export function AnimeModal({id}) {

    const [anime, setAnime] = useState({})
    const [editable, setEditable] = useState(false)
    const [curKeyword, setCurKeyword] = useState('')


    function fresh() {
        if (!id) return;
        fetch_anime_by_id(id).then(r => {
            setAnime(r)
        })
    }

    function commit_keyword() {
        add_keyword(anime.id, curKeyword).then(r => {
            console.log(r);
        }).finally(() => {
            fresh()
            setCurKeyword('')
        })
    }

    function del_keyword(id) {
        delete_keyword(id).finally(() => {
            fresh()
        })
    }

    useEffect(() => {
        fresh()
    }, [id])
    return (<>
        <div className="modal fade" id="view-modal" tabIndex="-1" aria-labelledby="modal-title-1"
             aria-hidden="true">
            <div className="modal-dialog">
                <div className="modal-content">
                    <div className="modal-header">
                        <div className="d-flex w-100">
                            <div className="w-100"><h1 className="modal-title fs-5"
                                                       id="modal-title-1">{editable ? "编辑" : "查看"}#{id}</h1>
                            </div>
                            <div className="flex-shrink-1">
                                <input type="checkbox" className="btn-check" id="example-checkbox-9"
                                       autoComplete="off" onChange={(e) => {
                                    if (editable) {
                                        console.log("del")
                                    } else {
                                        console.log("edit")
                                        setEditable(true)
                                    }
                                }}/>
                                <label className="btn btn-outline-primary"
                                       htmlFor="example-checkbox-9">{editable ? "删除" : "编辑"}</label>
                            </div>
                        </div>


                    </div>
                    <div className="modal-body">
                        <div className="mb-3">
                            <label htmlFor="example-input-1" className="form-label">名称</label>
                            <input className="form-control" id="example-input-1"
                                   placeholder="番剧名称" defaultValue={anime.title} readOnly={!editable}></input>
                        </div>
                        <div className="mb-3">
                            <label htmlFor="example-input-1" className="form-label">字幕组</label>
                            <input className="form-control" id="example-input-1"
                                   placeholder="字幕组" defaultValue={anime.sub} readOnly={!editable}></input>
                        </div>
                        <div className="mb-3">
                            <label htmlFor="example-textarea-1"
                                   className="form-label">RSS</label>
                            <textarea className="form-control" id="example-textarea-1" rows="3"
                                      type="url"
                                      placeholder="RSS链接" defaultValue={anime.rss} readOnly={!editable}></textarea>
                        </div>
                        <div>
                            <label htmlFor="example-textarea-2">
                                关键词过滤
                            </label>
                            <div>
                                <div className="hstack gap-1 my-1">
                                    {(anime.keywords || []).map((keyword, index) => <span key={index}
                                                                                          className="badge text-bg-danger show-hover"
                                                                                          onClick={() => del_keyword(keyword.id)}>{keyword.keyword}
                                        <i className="bi bi-x hide"></i>
                                        </span>)}
                                </div>

                                <div className={"input-group mb-3 " + (editable ? "" : "hide")}>
                                    <button type="button" className="btn btn-secondary dropdown-toggle"
                                            data-bs-toggle="dropdown" aria-expanded="false">排除
                                    </button>
                                    <ul className="dropdown-menu">
                                        <li><a className="dropdown-item active" href="#">排除</a></li>
                                        <li><a className="dropdown-item" href="#">包含</a></li>
                                    </ul>
                                    <input type="text" className="form-control" placeholder="关键词"
                                           aria-label="关键词" defaultValue={curKeyword}
                                           onChange={(e) => setCurKeyword(e.target.value)}/>
                                    <button type="button" className="btn btn-primary" id="button-add-on-1"
                                            disabled={!curKeyword} onClick={commit_keyword}>添加
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className="modal-footer">
                        <button type="button" className="btn btn-secondary" data-bs-dismiss="modal">取消
                        </button>
                        <button type="button" className="btn btn-primary">确定</button>
                    </div>
                </div>
            </div>
        </div>
    </>)
}