import axios from "axios";

const API_URL = ''

export default {
  register(username, password) {
    return axios.post(`${API_URL}/register`, { username, password });
  },
  
}
