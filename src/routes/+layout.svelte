<script lang="ts">
	import "fluent-svelte/theme.css";
	import { invoke } from "@tauri-apps/api/core";
	import { getCurrentWindow, getAllWindows, Window } from "@tauri-apps/api/window";
	import { onMount } from "svelte";

	onMount(async () => {
		check_accent();
		setInterval(() => {
			check_accent();
		}, 1000);
	});

	async function check_accent() {
		let argb: number[] = await invoke("get_accent_color");
		console.log(argb);
		let hsla = argb_to_accent_hsla(argb); //fixed lightness
		//--fds-accent-light-2
		document.documentElement.style.setProperty("--fds-accent-light-2", `${hsla[0]}, ${hsla[1]}%, ${hsla[2]}%`);
		//--fds-accent-dark-1
		document.documentElement.style.setProperty("--fds-accent-dark-1", `${hsla[0]}, ${hsla[1]}%, ${hsla[2] - 20}%`);
	}

	function argb_to_accent_hsla(argb: number[]): number[] {
		let r = argb[1] / 255;
		let g = argb[2] / 255;
		let b = argb[3] / 255;
		let a = argb[0] / 255;

		let max = Math.max(r, g, b);
		let min = Math.min(r, g, b);
		let l = (max + min) / 2;
		let s = 0;
		let h = 0;

		if (max != min) {
			s = l > 0.5 ? (max - min) / (2 - max - min) : (max - min) / (max + min);
			switch (max) {
				case r:
					h = (g - b) / (max - min) + (g < b ? 6 : 0);
					break;
				case g:
					h = (b - r) / (max - min) + 2;
					break;
				case b:
					h = (r - g) / (max - min) + 4;
					break;
			}
		}

		h = Math.round(h * 60);
		s = Math.round(s * 100);
		l = 63;

		return [h, s, l, a];
	}
</script>

<slot />

<style lang="postcss">
	* {
		margin: 0;
		padding: 0;
	}

	html,
	body {
		background-color: rgba(0, 0, 0, 10%);
		height: 100%;
		overflow: hidden;
	}
</style>
