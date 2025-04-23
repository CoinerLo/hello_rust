import { createRouter, createWebHistory } from "vue-router";
import RegisterForm from "@/components/RegisterForm.vue";
import RegisterForm from "@/components/Login.vue";
import RegisterForm from "@/components/Chat.vue";

const routes = [
  { path: '/register', component: RegisterForm },
  { path: '/login', component: Login },
  { path: '/chat/:chatId', component: Chat },
];

const router = createRouter({
  history: createWebHistory,
  routes,
});

export default router;
