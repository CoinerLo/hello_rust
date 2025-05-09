import io from 'socket.io-client';

const SOCKET_URL = 'https://127.0.0.1:8080';

const socket = io(SOCKET_URL);

export default {
  connect() {
    socket.connect();
  },

  disconnect() {
    socket.disconnect();
  },

  onMessage(callback) {
    socket.on('message', callback);
  },

  sendMessage(message) {
    socket.emit('message', message);
  },
};
