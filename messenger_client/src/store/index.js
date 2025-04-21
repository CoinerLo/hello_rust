import { createStore } from "vuex";
import api from "@/services/api"

export default createStore({
  state: {
    user: null,
    token: null,
    chats: [],
    messanges: {},
  },
  mutations: {
    setUser(state, user) {
      state.user = user;
    },
    setToken(state, token) {
      state.token = token;
    },
    setChats(state,chats) {
      state.chats = chats;
    },
    addMessage(state, { chatId, message }) {
      if (!state.messanges[chatId]) {
        state.messanges[chatId] = [];
      }
      state.messanges[chatId].push(message);
    },
  },
  actions: {
    async register({ commit }, { username, password }) {
      await api.register(username, password);
    },
    async login({ commit }, { username, password }) {
      const response = await api.login(username, password);
      commit('setUser', username);
      commit('setToken', response.data.token);
    },
    async fetchChats({ commit }) {
      const response = await api.getGroupChats();
      commit('setChats', response.data);
    },
    async sendMessage({commit}, { chatId, content }) {
      const response = await api.sendMessage(chatId, content);
      commit('addMessage', { chatId, message: response.data });
    },
    async createGroupChat({ commit }, name) {
      const response = await api.createGroupChat(name);
      commit('setChats', [ ...this.state.chats, response.data ]);
    },
    
  },
});
