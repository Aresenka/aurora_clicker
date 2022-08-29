import './style.css';
import { setupCounter } from './counter.js';
import MetaMaskOnboarding from '@metamask/onboarding';

const onboarding = new MetaMaskOnboarding();
onboarding.startOnboarding();


document.querySelector('#app').innerHTML = `
  <div>
    <h1>Aurora Clicker</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
  </div>
`;



setupCounter(document.querySelector('#counter'));
