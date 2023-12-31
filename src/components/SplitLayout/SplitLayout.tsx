import styles from './SplitLayout.module.css';

interface SplitLayoutProps {
  children: [React.ReactNode, React.ReactNode];
  gridTemplateColumns?: string;
}

const SplitLayout: React.FC<SplitLayoutProps> = ({
  children,
  gridTemplateColumns,
}) => {
  return (
    <div className={styles.container} style={{ gridTemplateColumns }}>
      {children}
    </div>
  );
};

export default SplitLayout;
