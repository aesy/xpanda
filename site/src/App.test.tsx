import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MantineProvider } from '@mantine/core';
import App from '@/App';
import { initI18n } from '@/i18n';
import { theme } from '@/theme/theme';

const renderApp = () =>
  render(
    <MantineProvider theme={theme} env="test">
      <App />
    </MantineProvider>
  );

beforeAll(() => initI18n());

describe('App', () => {
  it('expands the demo template with the demo variables', () => {
    renderApp();

    const output = screen.getByLabelText('Output');
    expect(output).toHaveTextContent('listen 8080;');
    expect(output).toHaveTextContent('server_name shop.example.org;');
    expect(output).toHaveTextContent('root /srv/webshop/public;');
    expect(output).toHaveTextContent('# v2.4.1 deployed by Alva');
    expect(output).toHaveTextContent('add_header X-Build "9f3c2e1";');
  });

  it('fails like the CLI when a required variable is missing', async () => {
    const user = userEvent.setup();
    renderApp();

    const vars = screen.getByLabelText('Variables');
    await user.clear(vars);
    await user.type(vars, 'APP=x');

    expect(screen.getByLabelText('Output')).toHaveTextContent('3:15 HOST must be set');
    expect(screen.getByText('exit 1')).toBeInTheDocument();
  });

  it('recomputes the pattern table for a new value', async () => {
    const user = userEvent.setup();
    renderApp();

    const input = screen.getByRole('textbox', { name: 'VAR=' });
    await user.clear(input);
    await user.type(input, 'panda');

    expect(screen.getByTestId('result-${VAR^^}')).toHaveTextContent('PANDA');
    expect(screen.getByTestId('result-${#VAR}')).toHaveTextContent('5');
  });

  it('shows errors in the pattern table when unset variables are not allowed', async () => {
    const user = userEvent.setup();
    renderApp();

    const patterns = within(
      screen.getByRole('heading', { name: 'Every form, evaluated live.' }).closest('section')!
    );
    await user.click(patterns.getByRole('checkbox', { name: 'unset' }));
    await user.click(patterns.getByRole('checkbox', { name: '-u (no unset)' }));

    expect(screen.getByTestId('result-${VAR}')).toHaveTextContent('error');
    expect(screen.getByTestId('result-${VAR:-default}')).toHaveTextContent('default');
  });
});
