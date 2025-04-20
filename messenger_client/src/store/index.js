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

  },
});
