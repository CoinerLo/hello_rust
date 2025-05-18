import axios from "axios";

const baseURL = 'http://127.0.0.1:8081'

const instance = axios.create({ baseURL });

instance.defaults.headers.common['Access-Control-Allow-Origin'] = '*';

export default {
  register(username, password) {
    return instance.post(`/register`, { username, password });
  },
  login(username, password) {
    return instance.post(`/authenticate`, { username, password });
  },
  sendMessage(chatId, content) {
    return instance.post(`/send-message`, { chatId, content });
  },
  addMemberToGroupChat(chatId, username) {
    return instance.post(`/add-member-to-group-chat`, { chatId, username });
  },
  removeMemberFroumGroupChat(chatId, username) {
    return instance.delete(`/remove-member-from-group-chat`, { chatId, username });
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
