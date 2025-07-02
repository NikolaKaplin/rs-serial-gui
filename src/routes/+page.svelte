<script lang="ts">
    import { onMount } from 'svelte';
    import { usePorts } from "$lib/hooks/usePorts";
    import type { Port } from "$lib/types/Port";

    let ports: Port[] = [];
    let selectedPort: Port | null = null;

    const loadPorts = async () => {
        ports = await usePorts();
    };

    const selectPort = (port: Port) => {
        selectedPort = port;
    };

    onMount(() => {
        loadPorts();
    });
</script>

<div class="min-h-screen flex flex-col lg:flex-row bg-gradient-to-b from-gray-900 to-gray-800 p-4 gap-4">

    <div class="flex-1 lg:flex-[0_0_350px] xl:flex-[0_0_400px] bg-gray-800 rounded-3xl border border-gray-700 flex flex-col overflow-hidden">
        <h3 class="text-center text-white py-4 text-xl sm:text-2xl border-b border-gray-700">Доступные порты</h3>
        <div class="grid grid-cols-2 p-3 text-white text-sm sm:text-base border-b border-gray-700">
            <span class="font-medium">Имя</span>
            <span class="font-medium text-right">Тип</span>
        </div>
        <div class="flex-1 overflow-y-auto">
            {#if ports}
                {#each ports as port}
                    <button on:click={() => selectPort(port)}
                            class={`w-full justify-between flex  p-3 text-sm cursor-pointer transition-colors ${
                            selectedPort?.port_name === port.port_name
                                ? 'bg-indigo-600 text-white'
                                : 'text-gray-300 hover:bg-gray-700/50'
                        }`}
                    >
                        <span class="font-medium truncate">{port.port_name}</span>
                        <span class="font-medium text-right">{port.port_type}</span>
                    </button>
                {/each}
            {/if}
        </div>
    </div>

    <div class="flex-1 bg-gray-800 rounded-3xl border border-gray-700 flex flex-col">
        <div class="border-b border-gray-700 p-4">
            <h3 class="text-white text-xl sm:text-2xl">
                {#if selectedPort}
                    <span class="text-indigo-400">{selectedPort.port_name}</span>
                {:else}
                    <span class="text-gray-400">Порт не выбран</span>
                {/if}
            </h3>
        </div>
        <div class="flex-1 flex items-center justify-center">
            {#if selectedPort}
                <div class="text-center p-6">
                    <h4 class="text-white text-lg mb-2">Информация о порте:</h4>
                    <div class="text-gray-300 grid grid-cols-2 gap-4 max-w-md mx-auto">
                        <div class="text-right">Имя:</div>
                        <div class="text-left font-medium">{selectedPort.port_name}</div>
                        <div class="text-right">Тип:</div>
                        <div class="text-left font-medium">{selectedPort.port_type}</div>
                    </div>
                </div>
            {:else}
                <div class="text-gray-400 text-lg">Выберите порт из списка слева</div>
            {/if}
        </div>
    </div>
</div>