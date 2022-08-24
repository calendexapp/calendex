<script>
	let grid = [96, 7];
	
	$: col = `repeat(${grid[1]}, 1fr)`;
	$: row = `repeat(${grid[0]}, 1fr)`;
	$: is_active = Array(grid[0]).fill(0).map(_ => Array(grid[1]).fill(false));
	
	let start = [];
	let end = [];
	let hover_end = []
	let clicked = false;
	
	function select(i, j) {
		if (clicked) {
			end = [i, j];
		} else { 
			start = [i, j];
		}
		
		clicked = !clicked;
		check_active([i, j]);
	}
	
	function hover(i, j) {
		if (!clicked) return;
		hover_end = [i, j];
		check_active(hover_end);
	}
	
	function is_in_range([i, j], [i2, j2]) {
		return ((i - start[0]) * (i - i2) <= 0) && 
			((j - start[1]) * (j - j2)<= 0)
	}
	
	function check_active (end) {
		is_active = is_active.map(
			(a, i) => a.map((_, j) => is_in_range([i, j], end)));
	}
	
	
</script>

<div class="container" style="grid-template-rows: {row}; grid-template-columns: {col};">

	{#each {length: grid[0]} as _, i (i)}
	  {#each {length: grid[1]} as _, j (j)}
			<div class:active={is_active[i][j]}
				on:click={() => select(i, j)}
				on:mouseover={() => hover(i, j)}
        class={i % 4 == 0 ? 'hour' : ''}
				></div>
		{/each}
	{/each}

</div>

<style>
	.container {
		display: grid;
		width: 100%;
		height: 1000px;
		grid-gap: 1px;
		background: #999;
	} 
	
  .hour {
    background: red; 
  }

	.container {
		background: #fff;
	}	
	
	div.active {
		background: orange;
	}

</style>
