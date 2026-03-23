import { type Component, createSignal, onCleanup } from 'solid-js';

export interface ResizeDraggerProps {
    direction: 'horizontal' | 'vertical';
    hidden: boolean;
    onResize: (delta: number) => void;
    onDoubleClick: () => void;
    accessibilityValue: number;
}

export const ResizeDragger: Component<ResizeDraggerProps> = (props) => {
    const [resizing, setResizing] = createSignal(false);

    const editBodyWithCursor = (enabled: boolean) => {
        document.body.style.cursor = enabled
            ? props.direction === 'horizontal'
                ? 'col-resize'
                : 'row-resize'
            : '';
        document.body.style.userSelect = enabled ? 'none' : '';
    };

    let lastPosition = 0;
    const handleMouseUp = () => {
        setResizing(false);
        editBodyWithCursor(false);
        window.removeEventListener('mouseup', handleMouseUp);
        window.removeEventListener('mousemove', handleMouseMove);
    };

    const handleMouseMove = (e: MouseEvent) => {
        const positionData =
            props.direction === 'horizontal' ? e.clientX : e.clientY;
        if (resizing()) {
            const delta = positionData - lastPosition;
            props.onResize(delta);
        }
        lastPosition = positionData;
    };

    onCleanup(() => {
        if (resizing()) {
            setResizing(false);
            editBodyWithCursor(false);
            window.removeEventListener('mouseup', handleMouseUp);
            window.removeEventListener('mousemove', handleMouseMove);
        }
    });

    return (
        <>
            {props.hidden ? null : (
                <div
                    class={`relative ${props.direction === 'horizontal' ? 'w-px h-full' : 'w-full h-px'} ${resizing() ? 'bg-zinc-600' : 'bg-zinc-600/20'}`}
                >
                    <div
                        aria-valuenow={props.accessibilityValue}
                        class={`absolute z-10 ${
                            props.direction === 'horizontal'
                                ? 'w-4 h-full top-0 -left-2 cursor-col-resize'
                                : 'h-4 w-full left-0 -top-2 cursor-row-resize'
                        }`}
                        onDblClick={() => {
                            props.onDoubleClick();
                        }}
                        onMouseDown={(e) => {
                            lastPosition =
                                props.direction === 'horizontal'
                                    ? e.clientX
                                    : e.clientY;
                            setResizing(true);
                            editBodyWithCursor(true);
                            window.addEventListener('mouseup', handleMouseUp);
                            window.addEventListener(
                                'mousemove',
                                handleMouseMove,
                            );
                        }}
                        role="slider"
                        tabIndex={0}
                    />
                </div>
            )}
        </>
    );
};
