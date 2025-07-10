<script lang="ts">
	import { usePorts } from '$lib/hooks/usePorts'
	import type { Port } from '$lib/types/Port'
	import { onMount } from 'svelte'

	let ports = $state<Port[]>([])
	let selectedPort = $state<Port | null>(null)

	const loadPorts = async () => {
		ports = await usePorts()
	}

	const selectPort = (port: Port) => {
		selectedPort = { ...port }
	}

	onMount(() => {
		loadPorts()
	})
</script>

<div
	class="min-h-screen flex flex-col lg:flex-row bg-gradient-to-b from-gray-900 to-gray-800 p-4 gap-4"
>
	<div
		class="flex-1 lg:flex-[0_0_350px] xl:flex-[0_0_400px] bg-gray-800 rounded-3xl border border-gray-700 flex flex-col overflow-hidden"
	>
		<h3
			class="text-center text-white py-4 text-xl sm:text-2xl border-b border-gray-700"
		>
			Доступные порты
		</h3>
		<div
			class="grid grid-cols-2 p-3 text-white text-sm sm:text-base border-b border-gray-700"
		>
			<span class="font-medium">Имя</span>
			<span class="font-medium text-right">Тип</span>
		</div>
		<div class="flex-1 overflow-y-auto">
			{#if ports}
				{#each ports as port}
					<button
						onclick={() => selectPort(port)}
						class={`w-full justify-between flex  p-3 text-sm cursor-pointer transition-colors ${
							selectedPort?.port_name === port.port_name
								? 'bg-indigo-600 text-white'
								: 'text-gray-300 hover:bg-gray-700/50'
						}`}
					>
						<span class="font-medium truncate">{port.port_name}</span>
						<span class="font-medium text-right"
							>{Object.keys(port.port_type)[0] == '0'
								? 'Нет информации'
								: Object.keys(port.port_type)[0]}</span
						>
					</button>
				{/each}
			{/if}
		</div>
	</div>

	<div
		class="flex-1 bg-gray-800 rounded-3xl border border-gray-700 flex flex-col overflow-hidden"
	>
		{#if selectedPort}
			<div class="p-6 space-y-6 animate-fade-in">
				<!-- Заголовок с иконкой -->
				<div class="flex items-center justify-center space-x-3">
					<svg
						class="w-8 h-8 text-indigo-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2z"
						/>
					</svg>
					<h3 class="text-2xl font-bold text-white">Детали порта</h3>
				</div>

				<!-- Карточка с информацией -->
				<div
					class="bg-gray-700/50 rounded-xl p-6 shadow-lg border border-gray-600/30 transition-all hover:border-indigo-400/30"
				>
					<!-- Основная информация в две колонки -->
					<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
						<!-- Колонка 1 -->
						<div class="space-y-4">
							<div>
								<p class="text-sm font-medium text-gray-400 mb-1">Имя порта</p>
								<p
									class="text-lg font-semibold text-white bg-gray-800/50 px-3 py-2 rounded-lg"
								>
									{selectedPort.port_name}
								</p>
							</div>

							<div>
								<p class="text-sm font-medium text-gray-400 mb-1">Тип порта</p>
								<div
									class="text-lg font-semibold text-white bg-gray-800/50 px-3 py-2 rounded-lg"
								>
									{#if typeof selectedPort.port_type === 'string'}
										<span class="text-green-400">{selectedPort.port_type}</span>
									{:else if selectedPort.port_type.UsbPort}
										<span class="flex items-center text-indigo-400">
											<svg
												class="w-5 h-5 mr-2"
												fill="currentColor"
												viewBox="0 0 20 20"
											>
												<path
													d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm0 12V5h12v10H4z"
												/>
											</svg>
											USB Порт
										</span>
									{:else}
										<span class="text-yellow-400">Неизвестный тип</span>
									{/if}
								</div>
							</div>
						</div>

						<!-- Колонка 2 -->
						<div class="space-y-4">
							<div>
								<p class="text-sm font-medium text-gray-400 mb-1">Статус</p>
								<div
									class="text-lg font-semibold text-white bg-gray-800/50 px-3 py-2 rounded-lg"
								>
									<span class="inline-flex items-center">
										<span class="flex w-3 h-3 bg-green-500 rounded-full mr-2"
										></span>
										Активен
									</span>
								</div>
							</div>

							<div>
								<p class="text-sm font-medium text-gray-400 mb-1">
									Последняя активность
								</p>
								<p
									class="text-lg font-semibold text-white bg-gray-800/50 px-3 py-2 rounded-lg"
								>
									Только что
								</p>
							</div>
						</div>
					</div>

					<!-- Дополнительная информация -->
					<div class="border-t border-gray-600/50 pt-6">
						<h4 class="text-lg font-bold text-white mb-4 flex items-center">
							<svg
								class="w-5 h-5 text-gray-400 mr-2"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
							Дополнительная информация
						</h4>

						{#if selectedPort.port_type.UsbPort}
							<div class="bg-gray-800/40 rounded-lg p-4">
								<ul class="space-y-3">
									{#each Object.entries(selectedPort.port_type.UsbPort) as [key, value]}
										<li
											class="flex justify-between items-center border-b border-gray-700/50 pb-2 last:border-0"
										>
											<span class="text-gray-400 font-medium">{key}:</span>
											<span class="text-white font-mono">{value}</span>
										</li>
									{/each}
								</ul>
							</div>
						{:else}
							<div class="text-center py-8 text-gray-500">
								<svg
									class="w-12 h-12 mx-auto mb-3"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="1.5"
										d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
									/>
								</svg>
								<p>Нет дополнительной информации</p>
							</div>
						{/if}
					</div>
				</div>
			</div>
		{:else}
			<div
				class="flex-1 flex flex-col items-center justify-center text-center p-8 animate-fade-in"
			>
				<svg
					class="w-16 h-16 text-gray-600 mb-4"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
					/>
				</svg>
				<h3 class="text-xl font-medium text-gray-400 mb-2">Порт не выбран</h3>
				<p class="text-gray-600 max-w-md">
					Выберите порт из списка слева, чтобы просмотреть подробную информацию
				</p>
			</div>
		{/if}
	</div>

	<style>
		.animate-fade-in {
			animation: fadeIn 0.3s ease-out forwards;
		}

		@keyframes fadeIn {
			from {
				opacity: 0;
				transform: translateY(10px);
			}
			to {
				opacity: 1;
				transform: translateY(0);
			}
		}
	</style>
</div>
