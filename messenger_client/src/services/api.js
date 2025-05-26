import axios from "axios";

const baseURL = 'http://127.0.0.1:8081'

const instance = axios.create({ baseURL });

instance.defaults.headers.common['Access-Control-Allow-Origin'] = '*';

export default {
  register(username, password) {
    return instance.post(`/register`, { username, password });
  },
  login(username, password) {
    return instance.post(`/login`, { username, password });
  },
  getGroupChats() {
    return instance.get(`/chats`);
  },
  createGroupChat(name, creator) {
    return instance.post(`/chats`, { name, creator });
  },
  deleteGroupChat(chatId) {
    return instance.delete(`/chats`, { chatId });
  },
}
