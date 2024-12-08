function fetch_anime() {
    return fetch('api/v1/anime').then(res => res.json());
}

function fetch_anime_by_id(id) {
    return fetch('api/v1/anime/' + id).then(res => res.json());
}

function add_anime(anime) {
    return fetch('api/v1/anime', {
        method: 'POST', body: JSON.stringify(anime)
    }).then(res => res.status >= 200 && res.status < 300);
}

function update_anime(anime) {
    return fetch('api/v1/anime', {
        method: 'PUT', body: JSON.stringify(anime)
    }).then(res => res.status >= 200 && res.status < 300);
}

function delete_anime(id) {
    return fetch('api/v1/anime/' + id, {
        method: 'DELETE',
    }).then(res => res.status >= 200 && res.status < 300);
}

function fetch_episodes_by_anime_id(anime_id) {
    return fetch('api/v1/anime/' + anime_id + '/episode').then(res => res.json());
}

function fetch_setting() {
    return fetch('api/v1/setting').then(res => res.json());
}

function update_setting(setting) {
    return fetch('api/v1/setting', {
        method: 'POST', body: JSON.stringify(setting)
    }).then(res => res.status >= 200 && res.status < 300);
}

function test_qb(qb) {
    return fetch('api/v1/test/qb', {
        method: 'POST', body: JSON.stringify(qb)
    }).then(res => res.status >= 200 && res.status < 300);
}

function fetch_keywords(anime_id) {
    return fetch('api/v1/anime/' + anime_id + '/keyword').then(res => res.json());
}

function add_keyword(anime_id, keyword) {
    return fetch('api/v1/anime/' + anime_id + '/keyword', {
        method: 'POST', body: JSON.stringify({keyword: keyword, anime_id})
    }).then(res => res.status >= 200 && res.status < 300);
}

function delete_keyword(id) {
    return fetch('api/v1/keyword/' + id, {
        method: 'DELETE'
    })
}

export {
    fetch_anime, fetch_anime_by_id, add_anime, update_anime, delete_anime,
    fetch_episodes_by_anime_id,
    fetch_keywords, add_keyword, delete_keyword,
    fetch_setting, update_setting,
    test_qb
}