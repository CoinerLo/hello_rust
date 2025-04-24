import { createRouter, createWebHistory } from "vue-router";
import RegisterForm from "@/components/RegisterForm.vue";
import LoginPage from "@/components/LoginPage.vue";
import ChatPage from "@/components/ChatPage.vue";

const routes = [
  { path: '/register', component: RegisterForm },
  { path: '/login', component: LoginPage },
  { path: '/chat/:chatId', component: ChatPage },
];

const router = createRouter({
  history: createWebHistory,
  routes,
});

export default router;
