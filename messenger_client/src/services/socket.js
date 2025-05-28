const SOCKET_URL = 'ws://127.0.0.1:8080';

let socket = null;

export default {
  connect(username) {
    socket = new WebSocket(SOCKET_URL);
    socket.onopen = () => {
      console.log('Соединение установлено');
      // Отправляем сообщение о присоединении
      socket.send(JSON.stringify({ type: 'Join', username }));
    };

    socket.onmessage = (event) => {
      console.log('Получено сообщение:', event.data);
    };

    socket.onclose = () => {
      console.log('Соединение закрыто');
    };

    socket.onerror = (error) => {
      console.error('Ошибка WebSocket:', error);
    };
  },

  disconnect() {
    socket.close();
  },

  onMessage(callback) {
    socket.onmessage = (event) => {
      callback(JSON.parse(event.data));
    };
  },

  sendMessage(message) {
    socket.send(JSON.stringify(message));
  },
};
