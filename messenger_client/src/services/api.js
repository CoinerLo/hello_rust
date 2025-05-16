import axios from "axios";

const API_URL = 'http://127.0.0.1:8081'

export default {
  register(username, password) {
    return axios.post(`${API_URL}/register`, { username, password });
  },
  login(username, password) {
    return axios.post(`${API_URL}/authenticate`, { username, password });
  },
  sendMessage(chatId, content) {
    return axios.post(`${API_URL}/send-message`, { chatId, content });
  },
  createGroupChat(name) {
    return axios.post(`${API_URL}/create-group-chat`, { name });
  },
  addMemberToGroupChat(chatId, username) {
    return axios.post(`${API_URL}/add-member-to-group-chat`, { chatId, username });
  },
  removeMemberFroumGroupChat(chatId, username) {
    return axios.delete(`${API_URL}/remove-member-from-group-chat`, { chatId, username });
  },
  deleteGroupChat(chatId) {
    return axios.delete(`${API_URL}/delete-group-chat`, { chatId });
  },
  getGroupChats() {
    return axios.get(`${API_URL}/chats`);
  },
}
