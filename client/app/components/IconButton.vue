<script setup lang="ts">
const props = withDefaults(defineProps<{
	type?: 'submit' | 'button' | 'reset' | 'link';
	size?: 'sm' | 'md';
	href?: string;
	disabled?: boolean;
}>(), {
	type: 'button',
	size: 'md',
});

const attrs = useAttrs();
const emit = defineEmits(['click']);
const classes = computed(() => [
	'iconButton',
	attrs.class,
	{ ['sm']: props.size === 'sm', ['md']: props.size === 'md', ['disabled']: props.disabled },
]);
</script>

<template>
	<NuxtLink
		v-if="type === 'link'"
		:to="props.href"
		:class="classes"
		@click="(e) => {
			if (props.disabled) { e.preventDefault() }
			else { emit('click', e) }
		}"
	>
		<slot />
	</NuxtLink>
	<button
		v-else
		:class="classes"
		:type="type"
		:disabled="props.disabled"
		@click="emit('click')"
	>
		<slot />
	</button>
</template>

<style scoped lang="scss">
a {
    box-sizing: border-box;
}
.iconButton {
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 100%;
    padding: $icon_button_padding;

    backdrop-filter: $blur;
    -webkit-backdrop-filter: $blur;
    border: $glass_border;
    background: $glass_item_background;
    box-shadow: $shadow_l2;
    transition: $transition;

    &.disabled {
        cursor: default;
        background-color: $glass_item_background_disabled;
        transform: scale(1);
        pointer-events: none;
    }

    &.md {
        height: 40px;
        width: 40px;
    }

    &.error {
        background: $error_glass_item_background;

        &:hover {
            background-color: $error_glass_item_background_focused;
        }
        &.disabled {
            background-color: $error_glass_item_background_disabled;
        }
    }

    :deep(svg) {
        height: 100%;
        width: 100%;
        fill: $placeholder_color;
        transition: $transition;
    }

    &:hover {
        background-color: $glass_item_background_focused;
        cursor: pointer;
        transform: scale(1.05);

        :deep(svg) {
            fill: $text_color;
        }
    }
}
</style>
