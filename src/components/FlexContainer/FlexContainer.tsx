interface FlexContainerProps {
  children: React.ReactNode;
  flexDirection?: 'row' | 'column' | 'row-reverse' | 'column-reverse';
  justifyContent?:
    | 'flex-start'
    | 'flex-end'
    | 'center'
    | 'space-between'
    | 'space-around';
  alignItems?: 'flex-start' | 'flex-end' | 'center' | 'baseline' | 'stretch';
  flexWrap?: 'nowrap' | 'wrap' | 'wrap-reverse';
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: string;
  width?: string;
  height?: string;
  padding?: string;
}

const FlexContainer: React.FC<FlexContainerProps> = ({
  children,
  flexDirection = 'row',
  justifyContent = 'flex-start',
  alignItems = 'flex-start',
  flexWrap = 'nowrap',
  flexGrow = 0,
  flexShrink = 1,
  flexBasis = 'auto',
  width = 'auto',
  height = 'auto',
  padding = '0',
}) => {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection,
        justifyContent,
        alignItems,
        flexWrap,
        flexGrow,
        flexShrink,
        flexBasis,
        width,
        height,
        padding,
      }}
    >
      {children}
    </div>
  );
};

export default FlexContainer;
