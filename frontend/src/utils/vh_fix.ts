export default function(): void {
  function setVh() {
    let vh = window.innerHeight * 0.01;
    document.documentElement.style.setProperty('--vh', `${vh}px`);
  }

  setVh();

  window.addEventListener('resize', () => {
    setVh();
  });
}