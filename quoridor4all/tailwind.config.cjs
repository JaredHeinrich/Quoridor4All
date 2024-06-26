/** @type {import('tailwindcss').Config}*/
const config = {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],

  plugins: [require('flowbite/plugin')],

  darkMode: 'class',

  theme: {
    extend: {
      colors: {
        // flowbite-svelte
        primary: {
          50: '#FFF5F2',
          100: '#FFF1EE',
          200: '#FFE4DE',
          300: '#FFD5CC',
          400: '#FFBCAD',
          500: '#FE795D',
          600: '#EF562F',
          700: '#EB4F27',
          800: '#CC4522',
          900: '#A5371B'
        }
      },

      gridTemplateColumns: {
        // Simple 14 column grid
        '14': 'repeat(14, minmax(0, 1fr))',
        '34': 'repeat(34, minmax(0, 1fr))',
        '44': 'repeat(44, minmax(0, 1fr))',
      },

      gridTemplateRows: {
        // Simple 14 column grid
        '14': 'repeat(14, minmax(0, 1fr))',
        '34': 'repeat(34, minmax(0, 1fr))',
        '44': 'repeat(44, minmax(0, 1fr))',
      }
    }
  }
};

module.exports = config;
