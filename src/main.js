import Vue from 'vue'
import App from './App.vue'
import svgJS from './plugins/svg'

Vue.use(svgJS)
Vue.config.productionTip = false
Vue.prototype.$wasm = import('./wasm-bootstrap.js')

new Vue({
  render: h => h(App),
  plugins: []
}).$mount('#app')
