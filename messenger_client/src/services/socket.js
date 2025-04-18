import io from 'socket.io-client';

const SOCKET_URL = '';

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
