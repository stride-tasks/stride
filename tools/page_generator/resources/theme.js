(() => {
  const root = document.documentElement;
  const themeToggle = document.getElementById('theme-toggle');
  const themeIcon = document.getElementById('theme-icon');

  if (!themeToggle || !themeIcon) {
    return;
  }

  const sunIcon = '<i class="fa-solid fa-sun"></i>';
  const moonIcon = '<i class="fa-solid fa-moon"></i>';

  const setTheme = (theme) => {
    const isDark = theme === 'dark';
    root.classList.toggle('dark', isDark);
    localStorage.setItem('stride-theme', theme);
    themeIcon.innerHTML = isDark ? sunIcon : moonIcon;
  };

  const preferredTheme = localStorage.getItem('stride-theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
  setTheme(preferredTheme);

  themeToggle.addEventListener('click', () => {
    const nextTheme = root.classList.contains('dark') ? 'light' : 'dark';
    setTheme(nextTheme);
  });
})();
