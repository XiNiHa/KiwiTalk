import { createResource, createEffect, on } from 'solid-js';
import { Navigate, Route, useNavigate } from '@solidjs/router';
import { useTransContext } from '@jellybrick/solid-i18next';

import { logon, autoLogin } from '@/api/api';
import { useConfig } from '@/features/config';

import { LoginPage } from './login';
import { MainPage } from './main';
import { ChannelListPage } from './main/channel';
import { LoginContentPage } from './login/login';
import { LoginListPage } from './login/list';
import { DeviceRegisterPage } from './login/device-register/page';

export const App = () => {
  const [t, { changeLanguage }] = useTransContext();
  const [config] = useConfig();
  const navigate = useNavigate();

  const [isLogin, { refetch }] = createResource(async () => {
    if (await logon()) return true;

    try {
      const res = await autoLogin();
      if (res.type === 'Success') {
        if (res.content) return true;
      } else {
        throw t(`login.status.login.${res.content}`);
      }
    } catch (err) {
      console.error(err);
      throw t(`login.reason.auto_login_failed`);
    }

    return false;
  });

  createEffect(on(config, (config) => {
    if (!config) return;

    if (config.global.locale.type === 'Auto') {
      changeLanguage(config.deviceLocale);
    } else {
      changeLanguage(config.global.locale.value);
    }
  }));

  createEffect(() => {
    if (isLogin() !== undefined) {
      if (isLogin()) navigate('/main');
      else navigate('/login', { state: { error: isLogin.error } });
    }
  });

  return (
    <>
      <Route path={'/main'} component={MainPage}>
        <Route path={'/'} element={<Navigate href={'/main/chat'} />} />
        <Route path={'/chat'}>
          <Route path={'/:channelId?'} component={ChannelListPage} />
        </Route>
        <Route path={'/*'} element={<div>TODO</div>} />
      </Route>
      <Route path={'/login'} component={LoginPage}>
        <Route path={'/'} component={LoginPage}/>
        <Route path={'/list'} component={LoginListPage} />
        <Route
          path={'/login'}
          component={LoginContentPage}
          data={() => refetch}
        />
        <Route path={'/device-register'} component={DeviceRegisterPage} />
      </Route>
    </>
  );
};
