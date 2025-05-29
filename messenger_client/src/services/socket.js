const SOCKET_URL = 'ws://127.0.0.1:8080';

let socket = null;

class WebSocketManager {
  constructor(store) {
    this.socket = null;
    this.store = store;
  }

  connect(username) {
    this.socket = new WebSocket(SOCKET_URL);

    this.socket.onopen = () => {
      console.log('Соединение установлено');
      // Отправляем сообщение о присоединении
      socket.send(JSON.stringify({ type: 'Join', username }));
    };

    this.socket.onmessage = (event) => {
      console.log('Получено сообщение:', event.data);
      const message = JSON.parse(event.data);
      this.handleIncomingMessage(message);
    };

    this.socket.onclose = () => {
      console.log('Соединение закрыто');
    };

    this.socket.onerror = (error) => {
      console.error('Ошибка WebSocket:', error);
    };
  }

  disconnect() {
    if (this.socket) {
      this.socket.close();
    }
  }

  sendMessage(message) {
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify(message));
    } else {
      console.error("WebSocket не подключен");
    }
  }

  handleIncomingMessage(message) {
    switch (message.type) {
      case 'ReceiveMessage':
        this.store.dispatch('addMessage', message);
        break;
      case 'ErrorMessage':
        console.error('Ошибка: ', message.error);
      default:
        console.warn("Неизвестный тип сообщения:", message.type);
    }
  }
}

export default function createWebSocketManager(store) {
  return new WebSocketManager(store);
}
