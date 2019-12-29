import Vue from 'vue'
import App from './App.vue'
import svgJS from './plugins/svg'

Vue.use(svgJS)
Vue.config.productionTip = false

new Vue({
  render: h => h(App),
  plugins: []
}).$mount('#app')
