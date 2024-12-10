import forms from "@tailwindcss/forms";
import typography from "@tailwindcss/typography";
import daisyui from 'daisyui';
import type { Config } from 'tailwindcss';

export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './static/**/*.svg'],

  theme: {
    extend: {}
  },

  plugins: [forms, typography, daisyui],
} satisfies Config;
