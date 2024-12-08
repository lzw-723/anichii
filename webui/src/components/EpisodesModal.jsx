import {fetch_episodes_by_anime_id} from "../api.js";
import {useEffect, useState} from "preact/hooks";

export function EpisodesModal({id}) {
    const [episodes, setEpisodes] = useState([])
    function fetch_episodes() {
        if(!id) return;
        fetch_episodes_by_anime_id(id).then(r => {
            setEpisodes(r)
        })
    }
    useEffect(() => {
        fetch_episodes()
    }, [id])
    return (<>
        <div className="modal fade modal-lg" id="eps-modal" tabIndex="-1" aria-labelledby="modal-title-1"
             aria-hidden="true">
            <div className="modal-dialog modal-dialog-scrollable">
                <div className="modal-content">
                    <div className="modal-header">
                        <h1 className="modal-title fs-5" id="modal-title-1">剧集</h1>
                    </div>
                    <div className="modal-body">
                        <table className="table table-striped">
                            <thead>
                            <tr>
                                <th scope="col">#</th>
                                <th scope="col">文件</th>
                                <th scope="col">下载</th>
                            </tr>
                            </thead>
                            <tbody>

                            {(episodes || []).map((ep, index) => <tr>
                                <th scope="row">{index + 1}</th>
                                <td>
                                    <a href={ep.torrent}>
                                        {ep.title}
                                    </a>
                                </td>
                                <td>
                                    {ep.ignore===true? '否' : '是'}
                                </td>
                            </tr>)}
                            </tbody>
                        </table>
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